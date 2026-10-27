// @api:src/lib.rs
// @impl:src/math_inner.rs
// @test:tests/ticket_9_additional_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
use pi_fixed_scale_int::{FixedI64, FixedU64};

fn within_one_raw(actual: i128, expected: i128) {
    assert!(
        (actual - expected).abs() <= 1,
        "raw={actual}, mathematical half-away rounded raw={expected}"
    );
}

pub fn negative_transcendentals_have_one_raw_error_bound() {
    type I = FixedI64<1_000_000>;
    type Operation = fn(I) -> Result<I, pi_fixed_scale_int::MathError>;
    // Independent mathematical constants rounded to six decimal places; no
    // strict API, backend result, or binary float is used as an oracle.
    let examples: [(i64, Operation, i128); 5] = [
        (-1_000_000, I::exp, 367_879),
        (-500_000, I::exp2, 707_107),
        (500_000, I::ln, -693_147),
        (750_000, I::log2, -415_037),
        (500_000, I::log10, -301_030),
    ];
    for (raw, operation, expected) in examples {
        let input = I::from_raw(raw);
        let result: I = operation(input).expect("finite in-range ordinary result");
        within_one_raw(i128::from(result.raw()), expected);
        assert_eq!(input.raw(), raw);
    }
}

pub fn maximum_denominator_negative_exponential_half_tie() {
    type I = FixedI64<{ u32::MAX }>;
    let input = I::from_raw(-i64::from(u32::MAX));
    let result: I = input.exp2().expect("2^-1 is representable after rounding");
    // (2^32 - 1) / 2 is exactly a positive half tie.
    within_one_raw(i128::from(result.raw()), 2_147_483_648);
    assert_eq!(input.raw(), -i64::from(u32::MAX));
}

pub fn powers_above_one_hundred_are_not_exponential_overflow() {
    for raw in [-101_000, 101_000] {
        let input = FixedI64::<1000>::from_raw(raw);
        for (exponent, expected) in [(1, i128::from(raw)), (-1, if raw < 0 { -10 } else { 10 })] {
            let result: FixedI64<1000> = input.powi(exponent).expect("power remains in range");
            // 1000 / 101 = 9 + 91/101, which rounds to 10.
            within_one_raw(i128::from(result.raw()), expected);
            assert_eq!(input.raw(), raw);
        }
    }
    let input = FixedU64::<1000>::from_raw(101_000);
    for (exponent, expected) in [(1, 101_000), (-1, 10)] {
        let result: FixedU64<1000> = input.powi(exponent).expect("power remains in range");
        within_one_raw(i128::from(result.raw()), expected);
        assert_eq!(input.raw(), 101_000);
    }
}
