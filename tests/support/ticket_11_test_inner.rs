use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};
// @api:src/lib.rs @impl:src/math_inner.rs @test:tests/ticket_11_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片11

pub fn negative_powers_preserve_sign_and_scale() {
    for raw in [-16_i64, -4, 4, 16] {
        for n in [-3_i32, -2] {
            // Exact independent rational: raw result = D^(k+1)/raw^k.
            let k = n.unsigned_abs();
            let numerator = 8_i128.pow(k + 1);
            let denominator = i128::from(raw).pow(k);
            assert_eq!(numerator % denominator, 0);
            let expected = numerator / denominator;
            let source = FixedI64::<8>::from_raw(raw);
            assert_eq!(i128::from(source.powi_strict(n).unwrap().raw()), expected);
            assert!((i128::from(source.powi(n).unwrap().raw()) - expected).abs() <= 1);
            assert_eq!(source.raw(), raw);
            if raw > 0 {
                let source = FixedU64::<8>::from_raw(raw as u64);
                assert_eq!(i128::from(source.powi_strict(n).unwrap().raw()), expected);
                assert!((i128::from(source.powi(n).unwrap().raw()) - expected).abs() <= 1);
                assert_eq!(source.raw(), raw as u64);
            }
        }
    }
}

pub fn huge_negative_power_never_returns_false_zero() {
    let signed = FixedI64::<1>::from_raw(i64::MAX);
    let unsigned = FixedU64::<1>::from_raw(u64::MAX);
    for strict in [false, true] {
        let a = if strict { signed.powi_strict(i32::MIN) } else { signed.powi(i32::MIN) };
        let b = if strict { unsigned.powi_strict(i32::MIN) } else { unsigned.powi(i32::MIN) };
        assert_eq!(a, Err(MathError::ComputationFailed));
        assert_eq!(b, Err(MathError::ComputationFailed));
        assert_eq!(signed.raw(), i64::MAX);
        assert_eq!(unsigned.raw(), u64::MAX);
    }
}

pub fn proven_exponential_extremes_respect_max_denominator() {
    for sign in [-1_i64, 1] {
        let raw = sign * 101 * i64::from(u32::MAX);
        let source = FixedI64::<{ u32::MAX }>::from_raw(raw);
        let expected = if sign < 0 { Ok(0) } else { Err(MathError::Overflow) };
        for result in [source.exp(), source.exp_strict(), source.exp2(), source.exp2_strict()] {
            assert_eq!(result.map(|value| value.raw()), expected);
            assert_eq!(source.raw(), raw);
        }
    }
}

pub fn unsigned_range_is_decided_after_rounding() {
    let small = FixedU64::<2>::from_raw(1);
    let base = FixedU64::<2>::from_raw(200);
    assert_eq!(small.log_strict(base).map(|value| value.raw()), Ok(0));
    assert_eq!(small.raw(), 1);
    assert_eq!(base.raw(), 200);
    let negative_result = FixedU64::<100>::from_raw(50);
    assert_eq!(negative_result.ln_strict(), Err(MathError::Overflow));
    assert_eq!(negative_result.raw(), 50);
}
