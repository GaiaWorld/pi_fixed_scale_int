use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64, ParseFixedError};
use std::str::FromStr;

pub fn decimal_strings_round_without_floating_point() {
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
}

pub fn invalid_or_out_of_range_text_is_rejected() {
    assert_eq!(
        FixedI64::<10>::from_str("nope"),
        Err(ParseFixedError::Invalid)
    );
    assert_eq!(
        FixedU64::<10>::from_str("-1"),
        Err(ParseFixedError::Overflow)
    );
    assert_eq!(
        FixedU64::<10>::from_str("18446744073709551616"),
        Err(ParseFixedError::Overflow)
    );
}

pub fn unsigned_negative_text_rounds_to_zero() {
    let text = String::from("-0.01");
    let before = text.clone();
    assert_eq!(FixedU64::<10>::from_str(&text).unwrap().raw(), 0);
    assert_eq!(text, before);
}

pub fn decimal_display_shows_expected_values() {
    assert_eq!(FixedU64::<100>::from_raw(123).to_string(), "1.23");
    assert_eq!(FixedU64::<3>::from_raw(3).to_string(), "1");
    assert_eq!(FixedI64::<10>::from_raw(-5).to_string(), "-0.5");
    assert_eq!(FixedI64::<10>::from_raw(0).to_string(), "0");
}

pub fn repeating_decimal_display_has_readable_decimal_format_only() {
    // 仅为格式验收，不能用本测试证明 1/3、2/3 的数值精度。
    for raw in [1, 2] {
        let source = FixedU64::<3>::from_raw(raw);
        let text = source.to_string();
        assert!(!text.is_empty());
        assert!(text.chars().all(|c| c.is_ascii_digit() || c == '.'));
        let value: f64 = text.parse().expect("readable decimal");
        assert!(value.is_finite(), "{text}");
        assert_eq!(source.raw(), raw);
    }
}

pub fn debug_includes_raw_and_denominator() {
    let unsigned = FixedU64::<100>::from_raw(123);
    let signed = FixedI64::<10>::from_raw(-12);
    // 数字分词不依赖类型名、字段标签、空格、字段顺序或标点。
    let unsigned_debug = format!("{unsigned:?}");
    let signed_debug = format!("{signed:?}");
    let unsigned_numbers: Vec<_> = unsigned_debug.split(|c: char| !c.is_ascii_digit() && c != '-').collect();
    let signed_numbers: Vec<_> = signed_debug.split(|c: char| !c.is_ascii_digit() && c != '-').collect();
    assert!(unsigned_numbers.contains(&"123") && unsigned_numbers.contains(&"100"), "{unsigned_debug}");
    assert!(signed_numbers.contains(&"-12") && signed_numbers.contains(&"10"), "{signed_debug}");
    assert_eq!(unsigned.raw(), 123);
    assert_eq!(signed.raw(), -12);
}

pub fn numeric_input_rounds_decimal_half_ties() {
    assert_eq!(FixedI64::<10>::try_from_f64(-1.25).unwrap().raw(), -13);
    assert_eq!(FixedU64::<10>::try_from(1.25f32).unwrap().raw(), 13);
    assert_eq!(FixedI64::<10>::try_from(-1.25f64).unwrap().raw(), -13);
}

pub fn nonfinite_numeric_input_is_rejected() {
    assert_eq!(
        FixedU64::<10>::try_from_f64(f64::NAN),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedI64::<10>::try_from_f64(f64::INFINITY),
        Err(ArithmeticError::Overflow)
    );
}

pub fn primitive_integer_input_scales_or_reports_overflow() {
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
}

pub fn finite_numeric_input_checks_final_range() {
    assert_eq!(FixedU64::<1>::try_from_f64(1.0e-100).unwrap().raw(), 0);
    assert_eq!(
        FixedU64::<1>::try_from_f64(1.0e100),
        Err(ArithmeticError::Overflow)
    );
}

pub fn mixed_operands_keep_left_type_and_scale() {
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
