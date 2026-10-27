use super::super::{
    Consts, Function, Interval, MathError, R, compute, divide, evaluate, integer, quantize,
};
const P: usize = 192;

// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片12
pub(super) fn logarithm_crossing_retries_or_exhausts_real_budget() {
    use super::super::{compute_with_budget, LOG_DENOMINATOR_TEST_BOUNDS};
    struct ClearQueue;
    impl Drop for ClearQueue {
        fn drop(&mut self) {
            LOG_DENOMINATOR_TEST_BOUNDS.with(|queue| queue.borrow_mut().clear());
        }
    }
    for strict in [false, true] {
        for (injected, budget, attempts, expected) in [
            (&[128][..], 128, &[128][..], Err(MathError::PrecisionExhausted)),
            (&[128][..], 256, &[128, 256][..], Ok(2)),
            (&[128, 256][..], 256, &[128, 256][..], Err(MathError::PrecisionExhausted)),
        ] {
            let _cleanup = ClearQueue;
            LOG_DENOMINATOR_TEST_BOUNDS.with(|queue| {
                let mut queue = queue.borrow_mut();
                queue.clear();
                for &p in injected {
                    // ln(2) is strictly between 0 and 1, so [-1,1] is a
                    // valid outward widening, never an invented error return.
                    queue.push_back((p, Interval { lo: integer(-1, p), hi: integer(1, p) }));
                }
            });
            reset_math_trace();
            let actual = compute_with_budget(4, 2, Function::Log(4), strict,
                0, u64::MAX.into(), 128, budget);
            let events = take_math_trace();
            assert_eq!(actual, expected, "strict={strict}, budget={budget}, injected={injected:?}");
            assert_adaptive_trace(&events, attempts, expected);
            LOG_DENOMINATOR_TEST_BOUNDS.with(|queue| assert!(queue.borrow().is_empty(),
                "all injected rounds must be consumed by real evaluate"));
            let aggregates = events.iter().filter(|event| matches!(event,
                MathTraceEvent::AdaptiveEntry { operation: "divide.aggregate" })).count();
            assert_eq!(aggregates, usize::from(expected.is_ok()));
        }
    }
}

// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片12
pub(super) fn closest_legal_log_bases_stay_strictly_off_zero() {
    use super::super::{compute_with_budget, ratio};
    const D: u32 = u32::MAX;
    let mut cc = Consts::new().unwrap();
    // For legal b != D, |b/D - 1| >= 1/D > 2^-32.
    // At p>=128, outward division error near 1 is below 2^-126;
    // neither endpoint can reach 1. Correctly directed ln preserves its
    // strict sign: |ln(b/D)| is far above the backend underflow floor.
    // The invalid b=D is rejected before evaluate (no legal crossing case).
    for base in [i128::from(D) - 1, i128::from(D) + 1] {
        for p in [128, 256] {
            let b = ratio(base, D, p).unwrap();
            let lo = b.lo.ln(p, R::Down, &mut cc);
            let hi = b.hi.ln(p, R::Up, &mut cc);
            let bounds = Interval { lo, hi };
            assert_eq!(bounds.validate(), Ok(()));
            let sign = if base < D.into() { -1 } else { 1 };
            assert_eq!(bounds.lo.cmp(&integer(0, p)), Some(sign));
            assert_eq!(bounds.hi.cmp(&integer(0, p)), Some(sign));
        }
        for strict in [false, true] {
            for budget in [128, 256] {
                reset_math_trace();
                let actual = compute_with_budget(base, D, Function::Log(base), strict,
                    0, u64::MAX.into(), 128, budget);
                let value = actual.unwrap();
                if strict { assert_eq!(value, D.into()); }
                else { assert!((value - i128::from(D)).abs() <= 1); }
                assert_adaptive_trace(&take_math_trace(), &[128], actual);
            }
        }
    }
    reset_math_trace();
    let actual = compute_with_budget(D.into(), D, Function::Log(D.into()), true,
        0, u64::MAX.into(), 128, 256);
    assert_eq!(actual, Err(MathError::Domain));
    assert_adaptive_trace(&take_math_trace(), &[], actual);
}

// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片12
pub(super) fn reciprocal_zero_endpoint_retries_real_loop_until_budget_exhausts() {
    use super::super::compute_with_budget;
    // 4^(-2^31) = 2^(-2^32), below even the unrestricted i32 exponent floor.
    assert!(-4_294_967_296_i64 < i64::from(astro_float::EXPONENT_MIN));
    for strict in [false, true] {
        for (budget, attempts) in [(128, &[128][..]), (256, &[128, 256][..])] {
            reset_math_trace();
            let result = compute_with_budget(1, 4, Function::Pow(i32::MIN), strict,
                0, u64::MAX.into(), 128, budget);
            assert_eq!(result, Err(MathError::PrecisionExhausted));
            let events = take_math_trace();
            assert_adaptive_trace(&events, attempts, result);
            assert!(!has_operation(&events, "power.reciprocal"));
            assert!(!has_operation(&events, "divide.aggregate"));
            assert!(!events.iter().any(|event| matches!(event,
                MathTraceEvent::BackendRequest { operation: "quantize.half.div", .. })));
        }
    }
}

// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片12
pub(super) fn ordinary_and_strict_diverge_then_strict_converges() {
    use super::super::compute_with_budget;
    const RAW: i128 = 5_483_946_715_995;
    const D: u32 = 4_113_749_459;
    let numerator = RAW.unsigned_abs().checked_pow(3).unwrap();
    let denominator = u128::from(D).pow(2);
    assert_eq!(2 * (numerator % denominator), denominator + 1);
    let expected = i128::try_from(numerator / denominator + 1).unwrap();
    assert_eq!(expected, 9_745_497_238_343_526_015);
    for (strict, budget, attempts) in [
        (false, 128, &[128][..]),
        (true, 128, &[128][..]),
        (true, 256, &[128, 256][..]),
    ] {
        reset_math_trace();
        let actual = compute_with_budget(RAW, D, Function::Pow(3), strict,
            0, u64::MAX.into(), 128, budget);
        if !strict {
            assert!((actual.unwrap() - expected).abs() <= 1);
        } else if budget == 128 {
            assert_eq!(actual, Err(MathError::PrecisionExhausted));
        } else {
            assert_eq!(actual, Ok(expected));
        }
        assert_adaptive_trace(&take_math_trace(), attempts, actual);
    }
}

// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片11
pub(super) fn range_straddling_retries_before_overflow() {
    use super::super::compute_with_budget;
    const RAW: i128 = 5_483_946_715_995;
    const D: u32 = 4_113_749_459;
    let numerator = (RAW as u128).checked_pow(3).unwrap();
    let denominator = u128::from(D).pow(2);
    assert_eq!(2 * (numerator % denominator), denominator + 1);
    let exact_rounded = i128::try_from(numerator / denominator + 1).unwrap();
    assert_eq!(exact_rounded, 9_745_497_238_343_526_015);
    for sign in [-1_i128, 1] {
        let expected = sign * exact_rounded;
        let (min, max) = if sign > 0 { (expected, u64::MAX.into()) }
            else { (-i128::from(u64::MAX), expected) };
        let mut cc = Consts::new().unwrap();
        let bounds = evaluate(sign * RAW, D, Function::Pow(3), 128, &mut cc).unwrap();
        let lo = quantize(&bounds.lo, 128).unwrap();
        let hi = quantize(&bounds.hi, 128).unwrap();
        if sign > 0 { assert!(lo < min && hi >= min, "{lo}..{hi}"); }
        else { assert!(lo <= max && hi > max, "{lo}..{hi}"); }
        for strict in [false, true] {
            for (budget, attempts, outcome) in [
                (128, &[128][..], Err(MathError::PrecisionExhausted)),
                (256, &[128, 256][..], Ok(expected)),
            ] {
                reset_math_trace();
                let actual = compute_with_budget(sign * RAW, D, Function::Pow(3),
                    strict, min, max, 128, budget);
                assert_eq!(actual, outcome);
                assert_adaptive_trace(&take_math_trace(), attempts, outcome);
            }
        }
    }
}

pub(super) fn backend_error_endpoints_are_explicitly_rejected() {
    let endpoint = astro_float::BigFloat::nan(Some(astro_float::Error::InvalidArgument));
    assert_eq!(endpoint.err(), Some(astro_float::Error::InvalidArgument));
    for lower in [false, true] {
        let bad = || Interval {
            lo: if lower { endpoint.clone() } else { integer(1, P) },
            hi: if lower { integer(2, P) } else { endpoint.clone() },
        };
        assert_eq!(bad().validate(), Err(MathError::ComputationFailed));
        assert_eq!(bad().require_positive(P), Err(MathError::ComputationFailed));
        assert!(matches!(divide(bad(), interval(1, 2), P), Err(MathError::ComputationFailed)));
        assert!(matches!(divide(interval(1, 2), bad(), P), Err(MathError::ComputationFailed)));
    }
}

fn has_operation(events: &[MathTraceEvent], name: &str) -> bool {
    events.iter().any(|event| matches!(event,
        MathTraceEvent::AdaptiveEntry { operation } if *operation == name))
}

pub(super) fn validation_precedes_aggregation_and_reciprocal() {
    reset_math_trace();
    assert!(divide(interval(2, 4), interval(2, 4), P).is_ok());
    assert!(has_operation(&take_math_trace(), "divide.aggregate"),
        "needs_inner_hook: positive control must observe real min/max point");
    let mut huge = integer(1, P);
    huge.set_exponent(astro_float::EXPONENT_MAX);
    let mut tiny = integer(1, P);
    tiny.set_exponent(astro_float::EXPONENT_MIN + 128);
    reset_math_trace();
    assert!(matches!(divide(Interval { lo: integer(0, P), hi: huge },
        Interval { lo: tiny.clone(), hi: tiny }, P), Err(MathError::ComputationFailed)));
    assert!(!has_operation(&take_math_trace(), "divide.aggregate"));
    let error = astro_float::BigFloat::nan(Some(astro_float::Error::InvalidArgument));
    for lower in [false, true] {
        for numerator in [false, true] {
            let bad = Interval {
                lo: if lower { error.clone() } else { integer(1, P) },
                hi: if lower { integer(2, P) } else { error.clone() },
            };
            reset_math_trace();
            let result = if numerator { divide(bad, interval(1, 2), P) }
                else { divide(interval(1, 2), bad, P) };
            assert!(matches!(result, Err(MathError::ComputationFailed)));
            assert!(!has_operation(&take_math_trace(), "divide.aggregate"));
        }
    }
    let mut cc = Consts::new().unwrap();
    reset_math_trace();
    assert!(evaluate(16, 8, Function::Pow(-3), P, &mut cc).is_ok());
    assert!(has_operation(&take_math_trace(), "power.reciprocal"),
        "needs_inner_hook: positive control must observe real reciprocal point");
    reset_math_trace();
    assert!(matches!(evaluate(u64::MAX.into(), 1, Function::Pow(i32::MIN), P, &mut cc),
        Err(MathError::ComputationFailed)));
    assert!(!has_operation(&take_math_trace(), "power.reciprocal"));
}

// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片11
pub(super) fn invalid_endpoints_fail_before_division_or_reciprocal() {
    let zero = integer(0, P);
    let one = integer(1, P);
    let nan = zero.div(&zero, P, R::Down);
    let infinity = one.div(&zero, P, R::Up);
    let mut cases = vec![interval(2, 1)];
    for endpoint in [nan, infinity.clone(), infinity.neg()] {
        cases.push(Interval { lo: endpoint.clone(), hi: one.clone() });
        cases.push(Interval { lo: one.clone(), hi: endpoint });
    }
    for bad in cases {
        assert_eq!(bad.require_positive(P), Err(MathError::ComputationFailed));
        let copy = Interval { lo: bad.lo.clone(), hi: bad.hi.clone() };
        assert!(matches!(divide(copy, interval(1, 2), P), Err(MathError::ComputationFailed)));
        assert!(matches!(divide(interval(1, 2), bad, P), Err(MathError::ComputationFailed)));
    }
}

// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 有限区间组合证明第4项
pub(super) fn division_covers_all_corner_signs() {
    // Expected endpoints are exact halves obtained independently from rational corners.
    for (a, b, twice_lo, twice_hi) in [
        ((2, 4), (2, 4), 1, 4),
        ((2, 4), (-4, -2), -4, -1),
        ((-4, -2), (2, 4), -4, -1),
        ((-4, -2), (-4, -2), 1, 4),
        ((-2, 4), (2, 4), -2, 4),
        ((-2, 4), (-4, -2), -4, 2),
    ] {
        let result = divide(interval(a.0, a.1), interval(b.0, b.1), P).unwrap();
        assert_eq!(result.validate(), Ok(()));
        let lo = integer(twice_lo, P).div(&integer(2, P), P, R::Down);
        let hi = integer(twice_hi, P).div(&integer(2, P), P, R::Up);
        assert_eq!(result.lo.cmp(&lo), Some(0));
        assert_eq!(result.hi.cmp(&hi), Some(0));
    }
}
// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片10–12
pub(super) fn strict_cubic_converges_only_after_precision_doubles() {
    use super::super::compute_with_budget;
    const RAW: u64 = 5_483_946_715_995;
    const D: u32 = 4_113_749_459;
    const EXPECTED: i128 = 9_745_497_238_343_526_015;
    // Independent exact rational raw result = RAW³/D². No backend, f64,
    // approximate decimal, or observed production result enters this oracle.
    let numerator = u128::from(RAW).checked_pow(3).unwrap();
    let denominator = u128::from(D).pow(2);
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    // Exact raw is quotient + 1/2 + 1/(2D²), strictly above the tie.
    assert_eq!(2 * remainder, denominator + 1);
    let oracle = i128::try_from(quotient + u128::from(2 * remainder >= denominator)).unwrap();
    assert_eq!(oracle, EXPECTED);

    for (budget, attempts, expected) in [
        (128, &[128][..], Err(MathError::PrecisionExhausted)),
        (256, &[128, 256][..], Ok(EXPECTED)),
    ] {
        reset_math_trace();
        let actual = compute_with_budget(i128::from(RAW), D, Function::Pow(3),
            true, 0, i128::from(u64::MAX), 128, budget);
        assert_eq!(actual, expected, "budget={budget}");
        // Requires actual evaluations at each precision and exactly one final
        // result event at the end, not repeated evaluate calls or fake traces.
        assert_adaptive_trace(&take_math_trace(), attempts, expected);
    }
}
pub(super) fn corner_failure_is_not_hidden_by_aggregation() {
    // Every input endpoint is finite and ordered. Only the quotient at the
    // large corner exceeds the backend exponent range; the other is zero.
    let mut huge = integer(1, P);
    huge.set_exponent(astro_float::EXPONENT_MAX);
    let mut tiny = integer(1, P);
    tiny.set_exponent(astro_float::EXPONENT_MIN + 128);
    for negative in [false, true] {
        let numerator = if negative {
            Interval { lo: huge.neg(), hi: integer(0, P) }
        } else {
            Interval { lo: integer(0, P), hi: huge.clone() }
        };
        let denominator = Interval { lo: tiny.clone(), hi: tiny.clone() };
        assert_eq!(numerator.validate(), Ok(()));
        assert_eq!(denominator.validate(), Ok(()));
        assert!(matches!(divide(numerator, denominator, P), Err(MathError::ComputationFailed)));
    }
}

// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片10–12
fn assert_adaptive_trace(events: &[MathTraceEvent], attempts: &[usize], expected: Result<i128, MathError>) {
    let actual: Vec<_> = events.iter().filter_map(|event| match event {
        MathTraceEvent::PrecisionAttempt { precision } => Some(*precision),
        _ => None,
    }).collect();
    assert_eq!(actual, attempts, "{events:?}");
    let results: Vec<_> = events.iter().filter_map(|event| match event {
        MathTraceEvent::PrecisionResult { result } => Some(*result),
        _ => None,
    }).collect();
    assert_eq!(results, [expected], "{events:?}");
    assert!(matches!(events.last(), Some(MathTraceEvent::PrecisionResult { result }) if *result == expected));
    let evaluations = events.iter().filter(|event| matches!(event,
        MathTraceEvent::AdaptiveEntry { operation: "evaluate" })).count();
    assert_eq!(evaluations, attempts.len(), "one real evaluation per attempt");
}

pub(super) fn real_budget_exhaustion_does_not_guess_half() {
    use super::super::compute_with_budget;
    // Independent exact identity D²/raw = +/-100/8 = +/-12.5.
    // The non-dyadic input +/-4/5 cannot be represented exactly at either budget.
    for raw in [-8, 8] {
        for (max, attempts) in [(128, &[128][..]), (256, &[128, 256][..])] {
            reset_math_trace();
            let result = compute_with_budget(raw, 10, Function::Pow(-1), true,
                i64::MIN.into(), i64::MAX.into(), 128, max);
            assert_eq!(result, Err(MathError::PrecisionExhausted));
            assert_adaptive_trace(&take_math_trace(), attempts, Err(MathError::PrecisionExhausted));
        }
    }
}

pub(super) fn production_entries_keep_default_precision() {
    macro_rules! check {
        ($ty:ident) => {
            for strict in [false, true] {
                let source = $ty::<1_000_000>::from_raw(1_000_000);
                reset_math_trace();
                let result = if strict { source.exp_strict() } else { source.exp() };
                let actual = result.map(|value| i128::from(value.raw())).expect("valid exp must succeed");
                if strict {
                    assert_eq!(actual, 2_718_282);
                } else {
                    assert!((actual - 2_718_282).abs() <= 1);
                }
                assert_adaptive_trace(&take_math_trace(), &[if strict { 192 } else { 128 }], Ok(actual));
                assert_eq!(source.raw(), 1_000_000);
            }
        };
    }
    check!(FixedI64);
    check!(FixedU64);
}

pub(super) fn limited_budget_preserves_error_categories() {
    use super::super::compute_with_budget;
    for (raw, d, function, expected, attempts) in [
        (0, 1, Function::Ln, Err(MathError::Domain), &[][..]),
        (2, 1, Function::Pow(64), Err(MathError::Overflow), &[128][..]),
        (i128::from(u64::MAX), 1, Function::Pow(i32::MIN), Err(MathError::ComputationFailed), &[128][..]),
        (1, 2, Function::Log(200), Ok(0), &[128][..]),
    ] {
        reset_math_trace();
        let result = compute_with_budget(raw, d, function, true, 0, u64::MAX.into(), 128, 256);
        assert_eq!(result, expected);
        assert_adaptive_trace(&take_math_trace(), attempts, expected);
    }
}


// @api:src/lib.rs @impl:src/math_inner.rs @test:src/math_inner_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片13及授权观测接缝
use super::super::{FixedI64, FixedU64, MathTraceEvent, reset_math_trace, take_math_trace};

fn assert_fixed_trace(events: &[MathTraceEvent]) {
    assert!(
        !events.is_empty(),
        "a real backend request must be observed"
    );
    for event in events {
        match event {
            MathTraceEvent::BackendRequest {
                operation,
                precision,
                rounding,
            } => {
                assert!(!operation.is_empty());
                assert_eq!(*precision, 128, "{operation}");
                assert!(matches!(rounding, R::ToEven), "{operation}: {rounding:?}");
            }
            MathTraceEvent::AdaptiveEntry { operation } => {
                panic!("fast method entered adaptive {operation}");
            }
            MathTraceEvent::PrecisionAttempt { precision } => {
                panic!("fast method entered adaptive precision attempt {precision}");
            }
            MathTraceEvent::PrecisionResult { result } => {
                panic!("fast method produced adaptive precision result {result:?}");
            }
        }
    }
}

fn assert_operations(events: &[MathTraceEvent], required: &[(&str, usize)]) {
    assert_fixed_trace(events);
    for &(name, minimum) in required {
        let count = events
            .iter()
            .filter(|event| {
                matches!(event,
                    MathTraceEvent::BackendRequest { operation, .. } if *operation == name
                )
            })
            .count();
        assert!(
            count >= minimum,
            "missing request {name}: expected at least {minimum}, got {count}: {events:?}"
        );
    }
}

fn observed<T: std::fmt::Debug + PartialEq>(
    call: impl FnOnce() -> Result<T, MathError>,
    expected: Result<T, MathError>,
    required: &[(&str, usize)],
) {
    reset_math_trace();
    assert_eq!(call(), expected);
    assert_operations(&take_math_trace(), required);
}

pub(super) fn fast_requests_use_only_fixed_budget() {
    macro_rules! check {
        ($ty:ident) => {{
            for method in 0..7 {
                reset_math_trace();
                let result = match method {
                    0 => $ty::<3>::from_raw(1).fast_exp(),
                    1 => $ty::<3>::from_raw(1).fast_exp2(),
                    2 => $ty::<3>::from_raw(6).fast_ln(),
                    3 => $ty::<3>::from_raw(6).fast_log2(),
                    4 => $ty::<3>::from_raw(30).fast_log10(),
                    5 => $ty::<3>::from_raw(12).fast_log($ty::from_raw(6)),
                    _ => $ty::<3>::from_raw(6).fast_powi(-1),
                };
                assert!(result.is_ok(), "method {method}: {result:?}");
                let events = take_math_trace();
                assert_operations(
                    &events,
                    &[
                        ("ratio.div", if method == 5 { 2 } else { 1 }),
                        ("scale.mul", 1),
                        ("quantize.half.div", 1),
                    ],
                );
                let required: &[(&str, usize)] = match method {
                    0 => &[("exp", 1)],
                    1 => &[("exp2.pow", 1)],
                    2 => &[("ln", 1)],
                    3 => &[("log2", 1)],
                    4 => &[("log10", 1)],
                    5 => &[("ln", 1), ("log.base.ln", 1), ("log.div", 1)],
                    _ => &[("powi", 1), ("power.reciprocal", 1)],
                };
                assert_operations(&events, required);
            }
        }};
    }
    check!(FixedI64);
    check!(FixedU64);
}

pub(super) fn fast_branch_requests_never_adapt() {
    observed(
        || FixedI64::<10>::from_raw(-20).fast_powi(3).map(|x| x.raw()),
        Ok(-80),
        &[
            ("ratio.div", 1),
            ("powi", 1),
            ("scale.mul", 1),
            ("quantize.half.div", 1),
        ],
    );
    observed(
        || FixedI64::<10>::from_raw(-20).fast_powi(-3).map(|x| x.raw()),
        Ok(-1),
        &[
            ("ratio.div", 1),
            ("powi", 1),
            ("power.reciprocal", 1),
            ("scale.mul", 1),
            ("quantize.half.div", 1),
        ],
    );
    macro_rules! check {
        ($ty:ident, $max:expr) => {{
            observed(
                || {
                    $ty::<100>::from_raw(100)
                        .fast_powi(i32::MIN)
                        .map(|x| x.raw())
                },
                Ok(100),
                &[
                    ("ratio.div", 1),
                    ("powi", 1),
                    ("power.reciprocal", 1),
                    ("scale.mul", 1),
                    ("quantize.half.div", 1),
                ],
            );
            observed(
                || $ty::<1>::from_raw(2).fast_powi(64),
                Err(MathError::Overflow),
                &[
                    ("ratio.div", 1),
                    ("powi", 1),
                    ("scale.mul", 1),
                    ("quantize.half.div", 1),
                ],
            );
            observed(
                || $ty::<1>::from_raw($max).fast_powi(i32::MIN),
                Err(MathError::ComputationFailed),
                &[("ratio.div", 1), ("powi", 1)],
            );
        }};
    }
    check!(FixedI64, i64::MAX);
    check!(FixedU64, u64::MAX);
}

pub(super) fn required_backend_operations_are_observed() {
    fast_requests_use_only_fixed_budget();
    observed(
        || FixedI64::<10>::from_raw(20).fast_exp2().map(|x| x.raw()),
        Ok(40),
        &[
            ("ratio.div", 1),
            ("exp2.powi", 1),
            ("scale.mul", 1),
            ("quantize.half.div", 1),
        ],
    );
    observed(
        || FixedU64::<10>::from_raw(20).fast_exp2().map(|x| x.raw()),
        Ok(40),
        &[
            ("ratio.div", 1),
            ("exp2.powi", 1),
            ("scale.mul", 1),
            ("quantize.half.div", 1),
        ],
    );
    observed(
        || FixedI64::<10>::from_raw(-20).fast_exp2().map(|x| x.raw()),
        Ok(3),
        &[
            ("ratio.div", 1),
            ("exp2.powi", 1),
            ("power.reciprocal", 1),
            ("scale.mul", 1),
            ("quantize.half.div", 1),
        ],
    );
}

pub(super) fn ordinary_and_strict_entries_are_observable() {
    macro_rules! check {
        ($ty:ident) => {{
            for strict in [false, true] {
                reset_math_trace();
                let x = $ty::<100>::from_raw(100);
                let result = if strict { x.exp2_strict() } else { x.exp2() };
                assert_eq!(result.map(|v| v.raw()), Ok(200));
                let events = take_math_trace();
                for expected in ["compute", "evaluate"] {
                    assert!(events.iter().any(|event| matches!(event,
                        MathTraceEvent::AdaptiveEntry { operation } if *operation == expected
                    )), "missing {expected}: {events:?}");
                }
            }
        }};
    }
    check!(FixedI64);
    check!(FixedU64);
}

pub(super) fn trace_read_and_reset_clear_events() {
    reset_math_trace();
    assert!(take_math_trace().is_empty());
    assert!(FixedI64::<3>::from_raw(1).fast_exp().is_ok());
    assert_fixed_trace(&take_math_trace());
    assert!(take_math_trace().is_empty());
    assert!(FixedI64::<3>::from_raw(1).fast_exp().is_ok());
    reset_math_trace();
    assert!(take_math_trace().is_empty());
}

pub(super) fn trace_buffers_are_thread_isolated() {
    use std::sync::mpsc;
    use std::time::Duration;
    const LIMIT: Duration = Duration::from_secs(10);
    reset_math_trace();
    let mut workers = Vec::new();
    for clear in [false, true] {
        let (ready_tx, ready_rx) = mpsc::channel();
        let (proceed_tx, proceed_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let handle = std::thread::spawn(move || {
            // catch_unwind always reports completion, including pre-sync assertion failures.
            let outcome = std::panic::catch_unwind(|| {
                assert!(take_math_trace().is_empty());
                assert!(FixedI64::<3>::from_raw(1).fast_exp().is_ok());
                ready_tx
                    .send(())
                    .expect("parent dropped readiness receiver");
                proceed_rx
                    .recv_timeout(LIMIT)
                    .expect("parent did not release worker");
                if clear {
                    reset_math_trace();
                }
                let events = take_math_trace();
                if clear {
                    assert!(events.is_empty());
                } else {
                    assert_fixed_trace(&events);
                }
                assert!(take_math_trace().is_empty());
            });
            let _ = done_tx.send(outcome.is_ok());
        });
        workers.push((handle, ready_rx, proceed_tx, done_rx));
    }
    // Never assert before releasing surviving workers. All receives have deadlines;
    // join is used only after a completion signal, not after a timeout.
    let ready = workers
        .iter()
        .all(|(_, rx, _, _)| rx.recv_timeout(LIMIT).is_ok());
    let parent_empty = take_math_trace().is_empty();
    for (_, _, tx, _) in &workers {
        let _ = tx.send(());
    }
    let mut successful = true;
    for (handle, _, _, rx) in workers {
        match rx.recv_timeout(LIMIT) {
            Ok(ok) => {
                successful &= ok;
                successful &= handle.join().is_ok();
            }
            Err(_) => {
                successful = false;
            } // detach instead of unbounded join
        }
    }
    assert!(
        ready && successful,
        "worker failed or synchronization timed out"
    );
    assert!(parent_empty, "child requests leaked into parent");
    assert!(take_math_trace().is_empty());
}

pub(super) fn reciprocal_half_remains_uncertified() {
    for raw in [-8, 8] {
        let expected = if raw < 0 { -13 } else { 13 };
        let mut cc = Consts::new().unwrap();
        let mut all_ambiguous = true;
        let mut p = 192;
        while p <= 12288 {
            let bounds = evaluate(raw, 10, Function::Pow(-1), p, &mut cc).unwrap();
            assert_eq!(bounds.validate(), Ok(()));
            // Exact binary half from the independent rational identity D^2/raw = +/-12.5.
            let half = integer(if raw < 0 { -25 } else { 25 }, p).div(&integer(2, p), p, R::Down);
            assert!(
                bounds.lo.cmp(&half).unwrap() <= 0,
                "lo exceeds true raw at p={p}"
            );
            assert!(
                bounds.hi.cmp(&half).unwrap() >= 0,
                "hi below true raw at p={p}"
            );
            let lo = quantize(&bounds.lo, p).unwrap();
            let hi = quantize(&bounds.hi, p).unwrap();
            assert!(lo <= expected && expected <= hi);
            if lo == hi {
                all_ambiguous = false;
            }
            p *= 2;
        }
        let actual = compute(
            raw,
            10,
            Function::Pow(-1),
            true,
            i128::from(i64::MIN),
            i128::from(i64::MAX),
        );
        if all_ambiguous {
            assert_eq!(actual, Err(MathError::PrecisionExhausted));
        } else {
            assert_eq!(actual, Ok(expected));
        }
    }
}

pub(super) fn explicit_precision_preserves_raw_bounds() {
    let mut cc = Consts::new().unwrap();
    for p in [128, 192, 384] {
        for (raw, function, expected) in [
            (1_000_000, Function::Exp, 2_718_282),
            (2_000_000, Function::Ln, 693_147),
            (0, Function::Pow(0), 1_000_000),
        ] {
            let bounds = evaluate(raw, 1_000_000, function, p, &mut cc).unwrap();
            assert_eq!(bounds.validate(), Ok(()));
            assert_eq!(quantize(&bounds.lo, p), Ok(expected), "lower p={p}");
            assert_eq!(quantize(&bounds.hi, p), Ok(expected), "upper p={p}");
        }
    }
}

fn interval(lo: i128, hi: i128) -> Interval {
    Interval {
        lo: integer(lo, P),
        hi: integer(hi, P),
    }
}
pub(super) fn invalid_intervals() {
    assert_eq!(interval(2, 1).validate(), Err(MathError::ComputationFailed));
    let nan = integer(0, P).div(&integer(0, P), P, R::Down);
    let infinity = integer(1, P).div(&integer(0, P), P, R::Up);
    for endpoint in [nan, infinity] {
        let bounds = Interval {
            lo: endpoint,
            hi: integer(1, P),
        };
        assert_eq!(bounds.validate(), Err(MathError::ComputationFailed));
    }
}
pub(super) fn zero_crossing_divisor() {
    for (lo, hi) in [(-1, 1), (0, 1)] {
        assert!(matches!(
            divide(interval(1, 2), interval(lo, hi), P),
            Err(MathError::PrecisionExhausted)
        ));
    }
}
pub(super) fn zero_before_reciprocal() {
    assert_eq!(
        interval(0, 1).require_positive(P),
        Err(MathError::PrecisionExhausted)
    );
}
pub(super) fn exact_half_quantization() {
    for (numerator, expected) in [(-3, -2), (-1, -1), (1, 1), (3, 2)] {
        let half = integer(numerator, P).div(&integer(2, P), P, R::Down);
        assert_eq!(quantize(&half, P), Ok(expected));
    }
    assert_eq!(
        quantize(&integer(i128::from(u64::MAX), P), P),
        Ok(i128::from(u64::MAX))
    );
}
