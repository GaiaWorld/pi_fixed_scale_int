// @api:src/lib.rs
// @impl:src/math_inner.rs
// @test:tests/ticket_9_power_edges_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};

fn within_one_raw(actual: i128, expected: i128) {
    assert!(
        (actual - expected).abs() <= 1,
        "raw={actual}, mathematically rounded raw={expected}"
    );
}

pub fn signed_odd_powers_below_minimum_overflow() {
    // Odd powers preserve the negative sign. Both exact integer results
    // are below -2^63; no backend or strict method supplies this oracle.
    for (raw, exponent) in [(-2, 65), (i64::MIN, 3)] {
        let input = FixedI64::<1>::from_raw(raw);
        assert_eq!(input.powi(exponent), Err(MathError::Overflow));
        assert_eq!(input.raw(), raw);
    }
}

pub fn signed_odd_power_at_minimum_succeeds() {
    let input = FixedI64::<1>::from_raw(-2);
    let result: FixedI64<1> = input.powi(63).expect("(-2)^63 equals the inclusive signed minimum");
    let expected = -(1_i128 << 63);
    within_one_raw(i128::from(result.raw()), expected);
    assert_eq!(input.raw(), -2);
}

pub fn large_integer_exponents_preserve_unit_magnitude() {
    // 1^n = 1, while (-1)^n is determined by parity, including negative n.
    // D=1000 separates the nonzero expected raw from a bogus underflow zero.
    for raw in [-1000, 1000] {
        for exponent in [101, -101, 102, -102] {
            let input = FixedI64::<1000>::from_raw(raw);
            let expected = if raw < 0 && exponent % 2 != 0 { -1000 } else { 1000 };
            let result: FixedI64<1000> = input.powi(exponent).expect("unit magnitude stays representable");
            within_one_raw(i128::from(result.raw()), expected);
            assert_eq!(input.raw(), raw);
        }
    }
    for exponent in [101, -101, 102, -102] {
        let input = FixedU64::<1000>::from_raw(1000);
        let result: FixedU64<1000> = input.powi(exponent).expect("unit magnitude stays representable");
        within_one_raw(i128::from(result.raw()), 1000);
        assert_eq!(input.raw(), 1000);
    }
}
