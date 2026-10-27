// @design-doc:design/pi-fixed-scale-int/00-index.md
// @test:tests/ticket_7_test.rs
// @api:src/lib.rs
// @impl:src/fixed_inner.rs
use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64, ParseFixedError};
use std::str::FromStr;

pub fn unsigned_negative_float_rounding_boundary() {
    // 十进制缩放分别为 -0.49、-0.5、-0.51；先舍入再判无符号范围。
    for (value, expected) in [(-0.049f32, Ok(0)), (-0.05, Err(ArithmeticError::Overflow)), (-0.051, Err(ArithmeticError::Overflow))] {
        let bits = value.to_bits();
        assert_eq!(FixedU64::<10>::try_from_f32(value).map(|v| v.raw()), expected, "direct f32 {value}");
        assert_eq!(FixedU64::<10>::try_from(value).map(|v| v.raw()), expected, "TryFrom f32 {value}");
        assert_eq!(value.to_bits(), bits);
    }
    for (value, expected) in [(-0.049f64, Ok(0)), (-0.05, Err(ArithmeticError::Overflow)), (-0.051, Err(ArithmeticError::Overflow))] {
        let bits = value.to_bits();
        assert_eq!(FixedU64::<10>::try_from_f64(value).map(|v| v.raw()), expected, "direct f64 {value}");
        assert_eq!(FixedU64::<10>::try_from(value).map(|v| v.raw()), expected, "TryFrom f64 {value}");
        assert_eq!(value.to_bits(), bits);
    }
}

pub fn text_rounding() {
    for (text, n) in [
        ("1.24", 12),
        ("-1.24", -12),
        ("1.25", 13),
        ("-1.25", -13),
        ("1.26", 13),
        ("-1.26", -13),
        ("0.05", 1),
        ("-0.05", -1),
    ] {
        let s = text.to_owned();
        let before = s.clone();
        assert_eq!(FixedI64::<10>::from_str(&s).unwrap().raw(), n, "{text}");
        assert_eq!(s, before);
    }
}

pub fn text_exponents_and_denominator() {
    assert_eq!(FixedI64::<100>::from_str("+1.25e2").unwrap().raw(), 12500);
    assert_eq!(FixedI64::<100>::from_str("-1.25E-2").unwrap().raw(), -1);
    assert_eq!(
        FixedU64::<{ u32::MAX }>::from_str("1").unwrap().raw(),
        u64::from(u32::MAX)
    );
}

pub fn invalid_text() {
    for text in [
        "", "+", "-", ".", "1.2.3", "1e", "1e2e3", " 1", "1 ", "nope", "NaN", "inf",
    ] {
        let s = text.to_owned();
        let before = s.clone();
        assert_eq!(
            FixedI64::<10>::from_str(&s),
            Err(ParseFixedError::Invalid),
            "{text:?}"
        );
        assert_eq!(
            FixedU64::<10>::from_str(&s),
            Err(ParseFixedError::Invalid),
            "{text:?}"
        );
        assert_eq!(s, before);
    }
}

pub fn text_rounded_bounds() {
    for (text, n) in [
        ("9223372036854775807.49", i64::MAX),
        ("-9223372036854775808.49", i64::MIN),
    ] {
        assert_eq!(FixedI64::<1>::from_str(text).unwrap().raw(), n);
    }
    for text in ["9223372036854775807.5", "-9223372036854775808.5"] {
        assert_eq!(
            FixedI64::<1>::from_str(text),
            Err(ParseFixedError::Overflow)
        );
    }
    assert_eq!(
        FixedU64::<1>::from_str("18446744073709551615.49")
            .unwrap()
            .raw(),
        u64::MAX
    );
    assert_eq!(
        FixedU64::<1>::from_str("18446744073709551615.5"),
        Err(ParseFixedError::Overflow)
    );
    assert_eq!(FixedU64::<10>::from_str("-0.049").unwrap().raw(), 0);
    assert_eq!(
        FixedU64::<10>::from_str("-0.05"),
        Err(ParseFixedError::Overflow)
    );
}

pub fn long_text() {
    let a = format!("{}1", "0".repeat(10001));
    let b = format!("1.25{}", "0".repeat(10001));
    let c = format!("5{}e-10003", "0".repeat(10001));
    assert_eq!(FixedU64::<10>::from_str(&a).unwrap().raw(), 10);
    assert_eq!(FixedI64::<10>::from_str(&b).unwrap().raw(), 13);
    assert_eq!(FixedU64::<10>::from_str(&c).unwrap().raw(), 1);
}

pub fn float_ties_zero_subnormal() {
    for (v, n) in [
        (1.25f64, 13),
        (-1.25, -13),
        (0.0, 0),
        (-0.0, 0),
        (f64::from_bits(1), 0),
        (-f64::from_bits(1), 0),
    ] {
        let bits = v.to_bits();
        assert_eq!(FixedI64::<10>::try_from_f64(v).unwrap().raw(), n);
        assert_eq!(FixedI64::<10>::try_from(v).unwrap().raw(), n);
        assert_eq!(v.to_bits(), bits);
    }
    for (v, n) in [
        (1.25f32, 13),
        (-1.25, -13),
        (0.0, 0),
        (-0.0, 0),
        (f32::from_bits(1), 0),
        (-f32::from_bits(1), 0),
    ] {
        let bits = v.to_bits();
        assert_eq!(FixedI64::<10>::try_from_f32(v).unwrap().raw(), n);
        assert_eq!(FixedI64::<10>::try_from(v).unwrap().raw(), n);
        assert_eq!(v.to_bits(), bits);
    }
}

pub fn f64_decimal_rounding() {
    // 最短文本 ±0.15 在 D10 恰为半值，不能改用精确二进制缩放。
    for (v, n) in [(0.15f64, 2), (-0.15, -2)] {
        let bits = v.to_bits();
        assert_eq!(
            FixedI64::<10>::try_from_f64(v).unwrap().raw(),
            n,
            "explicit {v}"
        );
        assert_eq!(FixedI64::<10>::try_from(v).unwrap().raw(), n, "TryFrom {v}");
        assert_eq!(v.to_bits(), bits);
    }
    assert_eq!(FixedU64::<10>::try_from(0.15f64).unwrap().raw(), 2);
    assert_eq!(FixedU64::<10>::try_from_f64(0.15).unwrap().raw(), 2);
}

pub fn f32_decimal_rounding() {
    // 保留 f32 的最短文本；先提升 f64 会把本例变成略小于半值。
    for (v, n) in [(0.35f32, 4), (-0.35, -4)] {
        let bits = v.to_bits();
        assert_eq!(
            FixedI64::<10>::try_from_f32(v).unwrap().raw(),
            n,
            "explicit {v}"
        );
        assert_eq!(FixedI64::<10>::try_from(v).unwrap().raw(), n, "TryFrom {v}");
        assert_eq!(v.to_bits(), bits);
    }
    assert_eq!(FixedU64::<10>::try_from(0.35f32).unwrap().raw(), 4);
    assert_eq!(FixedU64::<10>::try_from_f32(0.35).unwrap().raw(), 4);
}

pub fn large_float_decimal_value() {
    let v = 1_152_921_504_606_846_976f64;
    let bits = v.to_bits();
    assert_eq!(
        FixedU64::<1>::try_from(v).unwrap().raw(),
        1_152_921_504_606_847_000
    );
    assert_eq!(
        FixedI64::<1>::try_from_f64(-v).unwrap().raw(),
        -1_152_921_504_606_847_000
    );
    assert_eq!(FixedU64::<1>::try_from_f64(v).unwrap().raw(), 1_152_921_504_606_847_000);
    assert_eq!(FixedI64::<1>::try_from(-v).unwrap().raw(), -1_152_921_504_606_847_000);
    assert_eq!(v.to_bits(), bits);
}

pub fn nonfinite_float() {
    for v in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let b = v.to_bits();
        assert_eq!(
            FixedU64::<10>::try_from_f64(v),
            Err(ArithmeticError::Overflow)
        );
        assert_eq!(
            FixedI64::<10>::try_from_f64(v),
            Err(ArithmeticError::Overflow)
        );
        assert_eq!(FixedU64::<10>::try_from(v), Err(ArithmeticError::Overflow));
        assert_eq!(FixedI64::<10>::try_from(v), Err(ArithmeticError::Overflow));
        assert_eq!(v.to_bits(), b);
    }
    for v in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let b = v.to_bits();
        assert_eq!(
            FixedU64::<10>::try_from_f32(v),
            Err(ArithmeticError::Overflow)
        );
        assert_eq!(
            FixedI64::<10>::try_from_f32(v),
            Err(ArithmeticError::Overflow)
        );
        assert_eq!(FixedU64::<10>::try_from(v), Err(ArithmeticError::Overflow));
        assert_eq!(FixedI64::<10>::try_from(v), Err(ArithmeticError::Overflow));
        assert_eq!(v.to_bits(), b);
    }
}

pub fn float_range() {
    assert_eq!(
        FixedI64::<1>::try_from(-9_223_372_036_854_775_808f64),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedI64::<1>::try_from(9_223_372_036_854_775_808f64),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedU64::<1>::try_from(18_446_744_073_709_551_616f64),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedU64::<10>::try_from(-1.0f64),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedI64::<10>::try_from(f32::MAX),
        Err(ArithmeticError::Overflow)
    );
    for v in [-9_223_372_036_854_775_808f64, 9_223_372_036_854_775_808f64] {
        let bits = v.to_bits();
        assert_eq!(FixedI64::<1>::try_from_f64(v), Err(ArithmeticError::Overflow));
        assert_eq!(v.to_bits(), bits);
    }
    let v = 18_446_744_073_709_551_616f64;
    let bits = v.to_bits();
    assert_eq!(FixedU64::<1>::try_from_f64(v), Err(ArithmeticError::Overflow));
    assert_eq!(v.to_bits(), bits);
    let v = -1.0f64;
    let bits = v.to_bits();
    assert_eq!(FixedU64::<10>::try_from_f64(v), Err(ArithmeticError::Overflow));
    assert_eq!(v.to_bits(), bits);
    let v = f32::MAX;
    let bits = v.to_bits();
    assert_eq!(FixedI64::<10>::try_from_f32(v), Err(ArithmeticError::Overflow));
    assert_eq!(v.to_bits(), bits);
}

pub fn float_shortest_decimal_at_billion_scale() {
    // 期望来自合同给定的十进制数 0.1 × 1000000000，不借用解析器作浮点 oracle。
    for (v, expected) in [(0.1f32, 100_000_000), (-0.1f32, -100_000_000)] {
        let bits = v.to_bits();
        assert_eq!(FixedI64::<1_000_000_000>::try_from_f32(v).unwrap().raw(), expected);
        assert_eq!(FixedI64::<1_000_000_000>::try_from(v).unwrap().raw(), expected);
        assert_eq!(v.to_bits(), bits);
    }
    for (v, expected) in [(0.1f64, 100_000_000), (-0.1f64, -100_000_000)] {
        let bits = v.to_bits();
        assert_eq!(FixedI64::<1_000_000_000>::try_from_f64(v).unwrap().raw(), expected);
        assert_eq!(FixedI64::<1_000_000_000>::try_from(v).unwrap().raw(), expected);
        assert_eq!(v.to_bits(), bits);
    }
    for (text, expected) in [("0.1", 100_000_000), ("-0.1", -100_000_000)] {
        assert_eq!(FixedI64::<1_000_000_000>::from_str(text).unwrap().raw(), expected);
    }
    assert_eq!(FixedU64::<1_000_000_000>::try_from_f32(0.1).unwrap().raw(), 100_000_000);
    assert_eq!(FixedU64::<1_000_000_000>::try_from(0.1f32).unwrap().raw(), 100_000_000);
    assert_eq!(FixedU64::<1_000_000_000>::try_from_f64(0.1).unwrap().raw(), 100_000_000);
    assert_eq!(FixedU64::<1_000_000_000>::try_from(0.1f64).unwrap().raw(), 100_000_000);
    assert_eq!(FixedU64::<1_000_000_000>::from_str("0.1").unwrap().raw(), 100_000_000);
}

pub fn integer_inputs() {
    assert_eq!(FixedU64::<100>::try_from_integer(2).unwrap().raw(), 200);
    assert_eq!(FixedI64::<100>::try_from_number(-2i8).unwrap().raw(), -200);
    assert_eq!(FixedU64::<100>::try_from(2u16).unwrap().raw(), 200);
    let source = u64::MAX;
    assert_eq!(FixedU64::<1>::try_from(source).unwrap().raw(), source);
    assert_eq!(FixedI64::<1>::try_from(i64::MIN).unwrap().raw(), i64::MIN);
    assert_eq!(FixedI64::<1>::try_from(i64::MAX).unwrap().raw(), i64::MAX);
    assert_eq!(
        FixedU64::<2>::try_from(source),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(source, u64::MAX);
    assert_eq!(
        FixedI64::<2>::try_from_integer(i64::MIN),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedI64::<10>::try_from_number(i128::MAX),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedU64::<1>::try_from(u128::MAX),
        Err(ArithmeticError::Overflow)
    );
    assert_eq!(
        FixedU64::<10>::try_from(-1i8),
        Err(ArithmeticError::Overflow)
    );
}

pub fn decimal_display() {
    let a = FixedU64::<100>::from_raw(123);
    assert_eq!(a.to_string(), "1.23");
    assert_eq!(a.raw(), 123);
    let b = FixedI64::<10>::from_raw(-5);
    assert_eq!(b.to_string(), "-0.5");
    assert_eq!(b.raw(), -5);
    assert_eq!(FixedI64::<10>::from_raw(0).to_string(), "0");
    assert_eq!(
        FixedI64::<1>::from_raw(i64::MIN).to_string(),
        "-9223372036854775808"
    );
    assert_eq!(
        FixedU64::<1>::from_raw(u64::MAX).to_string(),
        "18446744073709551615"
    );
}

pub fn debug_raw_denominator() {
    let a = FixedU64::<100>::from_raw(123);
    let s = format!("{a:?}");
    assert!(s.split(|c: char| !c.is_ascii_digit() && c != '-').any(|token| token == "123"), "{s}");
    assert!(s.split(|c: char| !c.is_ascii_digit() && c != '-').any(|token| token == "100"), "{s}");
    assert_eq!(a.raw(), 123);
    let b = FixedI64::<10>::from_raw(-13);
    let s = format!("{b:?}");
    assert!(s.split(|c: char| !c.is_ascii_digit() && c != '-').any(|token| token == "-13"), "{s}");
    assert!(s.split(|c: char| !c.is_ascii_digit() && c != '-').any(|token| token == "10"), "{s}");
    assert_eq!(b.raw(), -13);
}
