use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64, ParseFixedError};
use std::str::FromStr;

#[test]
fn decimal_strings_round_without_floating_point() {
    assert_eq!(FixedI64::<10>::from_str("-1.25").unwrap().raw(), -13);
    assert_eq!(FixedI64::<10>::from_str("-0.05").unwrap().raw(), -1);
    assert_eq!(FixedU64::<100>::from_str("1.234").unwrap().raw(), 123);
    assert_eq!(FixedU64::<100>::from_str("1.235").unwrap().raw(), 124);
    assert_eq!(
        FixedU64::<{ u32::MAX }>::from_str("1").unwrap().raw(),
        u32::MAX as u64
    );
    assert_eq!(
        FixedI64::<1>::from_str("-9223372036854775808")
            .unwrap()
            .raw(),
        i64::MIN
    );
    assert_eq!(
        FixedU64::<1>::from_str("18446744073709551615")
            .unwrap()
            .raw(),
        u64::MAX
    );
    assert_eq!(FixedU64::<100>::from_str("1.25e2").unwrap().raw(), 12500);
    assert_eq!(FixedU64::<100>::from_str("0.001e1").unwrap().raw(), 1);
    assert_eq!(FixedU64::<100>::from_str("0.005e1").unwrap().raw(), 5);
    assert_eq!(FixedI64::<100>::from_str("-1.25e-2").unwrap().raw(), -1);
    assert_eq!(
        FixedI64::<10>::from_str("nope"),
        Err(ParseFixedError::Invalid)
    );
    assert_eq!(FixedU64::<10>::from_str("-0.01").unwrap().raw(), 0);
    assert_eq!(
        FixedU64::<10>::from_str("-1"),
        Err(ParseFixedError::Overflow)
    );
    assert_eq!(
        FixedU64::<10>::from_str("18446744073709551616"),
        Err(ParseFixedError::Overflow)
    );
}

#[test]
fn decimal_display_and_debug_show_expected_values() {
    assert_eq!(FixedU64::<100>::from_raw(123).to_string(), "1.23");
    assert_eq!(FixedU64::<3>::from_raw(3).to_string(), "1");
    assert_eq!(FixedI64::<10>::from_raw(-5).to_string(), "-0.5");
    assert_eq!(FixedI64::<10>::from_raw(0).to_string(), "0");
    assert_eq!(
        FixedU64::<3>::from_raw(1).to_string(),
        "0.333333333333333333"
    );
    assert_eq!(
        FixedU64::<3>::from_raw(2).to_string(),
        "0.666666666666666667"
    );
    assert_eq!(
        format!("{:?}", FixedU64::<100>::from_raw(123)),
        "FixedU64 { raw: 123, denominator: 100 }"
    );
    assert_eq!(
        format!("{:?}", FixedI64::<10>::from_raw(-12)),
        "FixedI64 { raw: -12, denominator: 10 }"
    );
}

#[test]
fn numeric_input_checks_non_finite_and_bounds() {
    assert_eq!(FixedI64::<10>::try_from_f64(-1.25).unwrap().raw(), -13);
    assert_eq!(FixedU64::<10>::try_from(1.25f32).unwrap().raw(), 13);
    assert_eq!(FixedI64::<10>::try_from(-1.25f64).unwrap().raw(), -13);
    assert_eq!(
        FixedU64::<10>::try_from_f64(f64::NAN),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedI64::<10>::try_from_f64(f64::INFINITY),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(FixedU64::<10>::try_from_number(100u8).unwrap().raw(), 1000);
    assert_eq!(FixedU64::<10>::try_from(100u8).unwrap().raw(), 1000);
    assert_eq!(FixedI64::<10>::try_from(100u64).unwrap().raw(), 1000);
    assert_eq!(
        FixedU64::<10>::try_from(u128::MAX),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(FixedI64::<10>::try_from_number(100u64).unwrap().raw(), 1000);
    assert_eq!(
        FixedU64::<10>::try_from_number(-1i8),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(FixedU64::<1>::try_from_f64(1.0e-100).unwrap().raw(), 0);
    assert_eq!(
        FixedU64::<1>::try_from_f64(1.0e100),
        Err(ArithmeticError::Overflow)
    );
}

#[test]
fn mixed_operands_keep_left_type_and_scale() {
    let signed = FixedI64::<100>::from_raw(150);
    let unsigned = FixedU64::<10>::from_raw(5);
    assert_eq!(signed.checked_mul_with(unsigned).unwrap().raw(), 75);
    assert_eq!(signed.checked_mul(unsigned).unwrap().raw(), 75);
    assert_eq!(signed.checked_div(unsigned).unwrap().raw(), 300);
    assert_eq!(signed.checked_div_with(unsigned).unwrap().raw(), 300);
    assert_eq!(signed.checked_mul_with(100u8).unwrap().raw(), 15000);
    assert_eq!(signed.checked_mul(100u8).unwrap().raw(), 15000);
    assert_eq!(signed.checked_div_with(2u8).unwrap().raw(), 75);
    assert_eq!(signed.checked_mul_with(-2i8).unwrap().raw(), -300);
    assert_eq!(
        FixedU64::<10>::from_raw(2).checked_mul_with(FixedI64::<2>::from_raw(-1)),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        signed.checked_div_with(0u8),
        Err(ArithmeticError::DivisionByZero)
    );
    assert_eq!(
        FixedI64::<10>::from_raw(-1)
            .rounded_div_with(4u8)
            .unwrap()
            .raw(),
        0
    );
    assert_eq!(
        FixedI64::<10>::from_raw(-1)
            .rounded_mul_with(FixedI64::<2>::from_raw(1))
            .unwrap()
            .raw(),
        -1
    );
    assert_eq!(
        FixedU64::<10>::from_raw(u64::MAX)
            .saturating_mul_with(2u8)
            .raw(),
        u64::MAX
    );
    assert_eq!(
        FixedI64::<1>::from_raw(i64::MIN).checked_mul(FixedU64::<1>::from_raw(u64::MAX)),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedI64::<1>::from_raw(i64::MIN)
            .saturating_mul(FixedU64::<1>::from_raw(u64::MAX))
            .raw(),
        i64::MIN
    );
    assert_eq!(
        FixedU64::<1>::from_raw(u64::MAX).checked_div(FixedI64::<1>::from_raw(-1)),
        Err(ArithmeticError::Overflow)
    );
}
