// @api:src/lib.rs @impl:src/math_inner.rs @test:tests/ticket_14_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片14
use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};

pub fn documented_fast_examples_keep_category_and_denominator() {
    macro_rules! check {
        ($ty:ident) => {{
            let zero = $ty::<100>::from_raw(0);
            let one = $ty::<100>::from_raw(100);
            let hundred = $ty::<100>::from_raw(10_000);
            let ten = $ty::<100>::from_raw(1_000);
            let two = $ty::<100>::from_raw(200);
            let results: [Result<$ty<100>, MathError>; 4] = [
                zero.fast_exp2(), one.fast_ln(), hundred.fast_log(ten), two.fast_powi(-1),
            ];
            for (actual, expected) in results.into_iter().zip([100, 0, 200, 50]) {
                assert_eq!(actual.unwrap().raw(), expected);
            }
            assert_eq!([zero.raw(), one.raw(), hundred.raw(), ten.raw(), two.raw()], [0, 100, 10_000, 1_000, 200]);
        }};
    }
    check!(FixedI64);
    check!(FixedU64);
    let x = FixedI64::<10>::from_raw(-20);
    let result: FixedI64<10> = x.fast_powi(3).unwrap();
    assert_eq!(result.raw(), -80);
    assert_eq!(x.raw(), -20);
}

// Independent reference decimals: e=2.718281828459..., sqrt(2)=1.414213562373...,
// ln(2)=0.693147180559..., log2(3)=1.584962500721...,
// log10(2)=0.301029995663...; (3/2)^3=27/8 exactly.
// These selected samples are not a uniform accuracy promise for arbitrary fast inputs.
pub fn representative_values_use_independent_decimal_references() {
    macro_rules! check {
        ($ty:ident) => {{
            let one = $ty::<1_000_000>::from_raw(1_000_000);
            let half = $ty::<1_000_000>::from_raw(500_000);
            let two = $ty::<1_000_000>::from_raw(2_000_000);
            let three = $ty::<1_000_000>::from_raw(3_000_000);
            let three_halves = $ty::<1_000_000>::from_raw(1_500_000);
            let results = [one.fast_exp(), half.fast_exp2(), two.fast_ln(), three.fast_log2(), two.fast_log10(), three.fast_log(two), three_halves.fast_powi(3)];
            for (actual, expected) in results.into_iter().zip([2_718_282, 1_414_214, 693_147, 1_584_963, 301_030, 1_584_963, 3_375_000]) {
                assert_eq!(actual.unwrap().raw(), expected);
            }
        }};
    }
    check!(FixedI64);
    check!(FixedU64);
}

fn exact_power_raw(raw: i128, d: i128, exponent: i32) -> i128 {
    let k = exponent.unsigned_abs();
    let (numerator, denominator) = if exponent >= 0 {
        (raw.pow(k) * d, d.pow(k))
    } else {
        (d.pow(k) * d, raw.pow(k))
    };
    let magnitude = numerator.abs();
    let divisor = denominator.abs();
    let rounded = magnitude / divisor + i128::from(2 * (magnitude % divisor) >= divisor);
    if (numerator < 0) != (denominator < 0) { -rounded } else { rounded }
}

pub fn binary_rational_powers_match_independent_integer_oracle() {
    // raw/8 is binary exact; selected reciprocals also have binary-exact half ties.
    // All integer oracle products fit i128; no floating-point or backend reference.
    for raw in [-24_i64, -20, -12, -4, 4, 12, 20, 24] {
        for exponent in -5..=5 {
            let expected = exact_power_raw(i128::from(raw), 8, exponent);
            let input = FixedI64::<8>::from_raw(raw);
            assert_eq!(i128::from(input.fast_powi(exponent).unwrap().raw()), expected, "raw={raw}, n={exponent}");
            assert_eq!(input.raw(), raw);
            if raw > 0 {
                let unsigned = FixedU64::<8>::from_raw(raw as u64);
                assert_eq!(i128::from(unsigned.fast_powi(exponent).unwrap().raw()), expected, "unsigned raw={raw}, n={exponent}");
                assert_eq!(unsigned.raw(), raw as u64);
            }
        }
    }
}

pub fn fast_domain_errors_preserve_operands() {
    macro_rules! check {
        ($ty:ident) => {{
            let zero = $ty::<100>::from_raw(0);
            let one = $ty::<100>::from_raw(100);
            let two = $ty::<100>::from_raw(200);
            for result in [zero.fast_ln(), zero.fast_log2(), zero.fast_log10(), zero.fast_log(two), two.fast_log(zero), two.fast_log(one), zero.fast_powi(i32::MIN)] {
                assert_eq!(result, Err(MathError::Domain));
            }
            assert_eq!([zero.raw(), one.raw(), two.raw()], [0, 100, 200]);
        }};
    }
    check!(FixedI64);
    check!(FixedU64);
    let negative = FixedI64::<100>::from_raw(-100);
    let two = FixedI64::<100>::from_raw(200);
    for result in [negative.fast_ln(), negative.fast_log2(), negative.fast_log10(), negative.fast_log(two), two.fast_log(negative)] {
        assert_eq!(result, Err(MathError::Domain));
    }
    assert_eq!((negative.raw(), two.raw()), (-100, 200));
}

pub fn fast_range_errors_preserve_operands() {
    let signed = FixedI64::<1>::from_raw(2);
    let unsigned = FixedU64::<1>::from_raw(2);
    assert_eq!(signed.fast_powi(64), Err(MathError::Overflow));
    assert_eq!(unsigned.fast_powi(64), Err(MathError::Overflow));
    assert_eq!(signed.raw(), 2);
    assert_eq!(unsigned.raw(), 2);
    let half = FixedU64::<100>::from_raw(50);
    assert_eq!(half.fast_ln(), Err(MathError::Overflow));
    assert_eq!(half.raw(), 50);
}

pub fn fast_invalid_intermediate_errors_preserve_operands() {
    let signed = FixedI64::<1>::from_raw(i64::MAX);
    let unsigned = FixedU64::<1>::from_raw(u64::MAX);
    assert_eq!(signed.fast_powi(i32::MIN), Err(MathError::ComputationFailed));
    assert_eq!(unsigned.fast_powi(i32::MIN), Err(MathError::ComputationFailed));
    assert_eq!(signed.raw(), i64::MAX);
    assert_eq!(unsigned.raw(), u64::MAX);
}

pub fn fast_nonidentity_odd_power_at_signed_minimum_succeeds() {
    // Exact integer bases and results fit the 128-bit backend budget; no
    // mathematical approximation or other math method supplies this oracle.
    for (raw, exponent) in [(-2_097_152_i64, 3_u32), (-2, 63)] {
        let expected = i128::from(raw).pow(exponent);
        assert_eq!(expected, i128::from(i64::MIN));
        let input = FixedI64::<1>::from_raw(raw);
        let actual: FixedI64<1> = input.fast_powi(exponent as i32).unwrap();
        assert_eq!(i128::from(actual.raw()), expected, "raw={raw}, n={exponent}");
        assert_eq!(input.raw(), raw);
    }
}

pub fn fast_adjacent_odd_power_below_signed_minimum_overflows() {
    // The cube's base differs by one raw from the exact inclusive-boundary case.
    // Both exact integer results fit i128 and are below the storage lower bound.
    for (raw, exponent) in [(-2_097_153_i64, 3_u32), (-2, 65)] {
        let expected = i128::from(raw).pow(exponent);
        assert!(expected < i128::from(i64::MIN));
        let input = FixedI64::<1>::from_raw(raw);
        assert_eq!(input.fast_powi(exponent as i32), Err(MathError::Overflow), "exact raw={expected}");
        assert_eq!(input.raw(), raw);
    }
}

pub fn ordinary_and_strict_still_match_independent_references() {
    macro_rules! check {
        ($ty:ident) => {{
            let one = $ty::<1_000_000>::from_raw(1_000_000);
            let half = $ty::<1_000_000>::from_raw(500_000);
            let two = $ty::<1_000_000>::from_raw(2_000_000);
            let three = $ty::<1_000_000>::from_raw(3_000_000);
            let three_halves = $ty::<1_000_000>::from_raw(1_500_000);
            let ordinary = [one.exp(), half.exp2(), two.ln(), three.log2(), two.log10(), three.log(two), three_halves.powi(3)];
            let strict = [one.exp_strict(), half.exp2_strict(), two.ln_strict(), three.log2_strict(), two.log10_strict(), three.log_strict(two), three_halves.powi_strict(3)];
            for ((ordinary, strict), expected) in ordinary.into_iter().zip(strict).zip([2_718_282_i128, 1_414_214, 693_147, 1_584_963, 301_030, 1_584_963, 3_375_000]) {
                assert_eq!(i128::from(strict.unwrap().raw()), expected);
                assert!((i128::from(ordinary.unwrap().raw()) - expected).abs() <= 1);
            }
            assert_eq!([one.raw(), half.raw(), two.raw(), three.raw(), three_halves.raw()], [1_000_000, 500_000, 2_000_000, 3_000_000, 1_500_000]);
        }};
    }
    check!(FixedI64);
    check!(FixedU64);
}
