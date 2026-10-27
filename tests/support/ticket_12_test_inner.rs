// @api:src/lib.rs @impl:src/math_inner.rs @test:tests/ticket_12_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片12
use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};

// Oracle uses only exact integer arithmetic, independently of the math backend.
fn oracle(raw: i128, d: u32, n: i32) -> i128 {
    if n == 0 { return i128::from(d); }
    let k = n.unsigned_abs();
    let magnitude = raw.unsigned_abs();
    let (numerator, denominator) = if n > 0 {
        (magnitude.checked_pow(k).unwrap(), u128::from(d).checked_pow(k - 1).unwrap())
    } else {
        (u128::from(d).checked_pow(k + 1).unwrap(), magnitude.checked_pow(k).unwrap())
    };
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    // Compare r >= denominator-r rather than multiplying a potentially large r.
    // All stored results are at most u64::MAX in magnitude. Preserve an
    // out-of-range sentinel rather than narrowing a valid u128 oracle to i128.
    let magnitude_limit = u128::from(u64::MAX) + 1;
    let rounded = i128::try_from((quotient + u128::from(remainder >= denominator - remainder)).min(magnitude_limit)).unwrap();
    if raw < 0 && k % 2 == 1 { -rounded } else { rounded }
}

fn check(result: Result<i128, MathError>, expected: i128, strict: bool, min: i128, max: i128) {
    if expected < min || expected > max {
        assert_eq!(result, Err(MathError::Overflow), "expected raw {expected}");
    } else {
        match result {
            Ok(actual) if strict => assert_eq!(actual, expected),
            Ok(actual) => assert!((actual - expected).abs() <= 1, "{actual} vs {expected}"),
            Err(MathError::PrecisionExhausted) if strict => {},
            other => panic!("unexpected {other:?}, expected raw {expected}, strict={strict}"),
        }
    }
}

fn grid<const D: u32>() {
    let d = i128::from(D);
    for magnitude in [1, d / 2, d - 1, d, d + 1, 2 * d] {
        for n in -3..=3 {
            for sign in [-1, 1] {
                let raw = sign * magnitude;
                let input = FixedI64::<D>::from_raw(i64::try_from(raw).unwrap());
                let expected = oracle(raw, D, n);
                for strict in [false, true] {
                    let result: Result<FixedI64<D>, _> = if strict { input.powi_strict(n) } else { input.powi(n) };
                    check(result.map(|v| i128::from(v.raw())), expected, strict, i64::MIN.into(), i64::MAX.into());
                    assert_eq!(i128::from(input.raw()), raw);
                }
            }
            let raw = u64::try_from(magnitude).unwrap();
            let input = FixedU64::<D>::from_raw(raw);
            let expected = oracle(magnitude, D, n);
            for strict in [false, true] {
                let result: Result<FixedU64<D>, _> = if strict { input.powi_strict(n) } else { input.powi(n) };
                check(result.map(|v| i128::from(v.raw())), expected, strict, 0, u64::MAX.into());
                assert_eq!(input.raw(), raw);
            }
        }
    }
}

pub(super) fn powers_match_independent_integer_oracle() {
    grid::<8>();
    grid::<97>();
    grid::<{ u32::MAX }>();
}

pub(super) fn near_half_cubic_has_independent_64_bit_answer() {
    const RAW: u64 = 5_483_946_715_995;
    const D: u32 = 4_113_749_459;
    let numerator = u128::from(RAW).pow(3);
    let denominator = u128::from(D).pow(2);
    assert_eq!(2 * (numerator % denominator), denominator + 1);
    let expected = oracle(RAW.into(), D, 3);
    assert_eq!(expected, 9_745_497_238_343_526_015);
    let source = FixedU64::<D>::from_raw(RAW);
    let strict: FixedU64<D> = source.powi_strict(3).unwrap();
    assert_eq!(i128::from(strict.raw()), expected);
    check(source.powi(3).map(|v| v.raw().into()), expected, false, 0, u64::MAX.into());
    assert_eq!(source.raw(), RAW);
}
