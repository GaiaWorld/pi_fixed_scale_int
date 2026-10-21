#![cfg(feature = "math")]

use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};

type Number = FixedI64<1_000_000>;

#[test]
fn known_transcendental_results_are_correctly_rounded() {
    let one = Number::from_raw(1_000_000);
    assert_eq!(one.exp_strict().unwrap().raw(), 2_718_282);
    assert_eq!(
        Number::from_raw(2_000_000).ln_strict().unwrap().raw(),
        693_147
    );
    assert_eq!(
        Number::from_raw(500_000).ln_strict().unwrap().raw(),
        -693_147
    );
    assert_eq!(
        Number::from_raw(500_000).exp2_strict().unwrap().raw(),
        1_414_214
    );
    assert_eq!(
        Number::from_raw(3_000_000).log2_strict().unwrap().raw(),
        1_584_963
    );
    assert_eq!(
        Number::from_raw(2_000_000).log10_strict().unwrap().raw(),
        301_030
    );
    assert_eq!(
        Number::from_raw(8_000_000)
            .log_strict(Number::from_raw(2_000_000))
            .unwrap()
            .raw(),
        3_000_000
    );
    assert_eq!(
        Number::from_raw(4_000_000)
            .log_strict(Number::from_raw(500_000))
            .unwrap()
            .raw(),
        -2_000_000
    );
}

#[test]
fn exact_values_and_powers_include_half_away_ties() {
    assert_eq!(Number::from_raw(0).exp_strict().unwrap().raw(), 1_000_000);
    assert_eq!(Number::from_raw(1_000_000).ln_strict().unwrap().raw(), 0);
    assert_eq!(
        Number::from_raw(2_000_000).powi_strict(-3).unwrap().raw(),
        125_000
    );
    assert_eq!(
        Number::from_raw(-2_000_000).powi_strict(3).unwrap().raw(),
        -8_000_000
    );
    assert_eq!(Number::from_raw(0).powi_strict(0).unwrap().raw(), 1_000_000);
    assert_eq!(FixedI64::<10>::from_raw(5).powi_strict(2).unwrap().raw(), 3);
    assert_eq!(
        FixedI64::<1>::from_raw(-2).powi_strict(-1).unwrap().raw(),
        -1
    );
    assert_eq!(FixedI64::<1>::from_raw(-1).exp2_strict().unwrap().raw(), 1);
}

#[test]
fn domain_overflow_underflow_and_unsigned_results() {
    assert_eq!(Number::from_raw(0).ln_strict(), Err(MathError::Domain));
    assert_eq!(Number::from_raw(-1).log2(), Err(MathError::Domain));
    assert_eq!(
        Number::from_raw(1).log(Number::from_raw(1_000_000)),
        Err(MathError::Domain)
    );
    assert_eq!(
        Number::from_raw(1).log(Number::from_raw(0)),
        Err(MathError::Domain)
    );
    assert_eq!(Number::from_raw(0).powi(-1), Err(MathError::Domain));
    assert_eq!(
        Number::from_raw(i64::MAX).exp_strict(),
        Err(MathError::Overflow)
    );
    assert_eq!(Number::from_raw(i64::MIN).exp_strict().unwrap().raw(), 0);
    assert_eq!(
        FixedU64::<100>::from_raw(50).ln_strict(),
        Err(MathError::Overflow)
    );
    assert_eq!(
        FixedU64::<100>::from_raw(100).exp_strict().unwrap().raw(),
        272
    );
}

#[test]
fn approximate_and_strict_methods_respect_one_raw_contract() {
    for raw in [-2_500_000, -500_000, 0, 500_000, 3_250_000] {
        let x = Number::from_raw(raw);
        assert!((x.exp().unwrap().raw() - x.exp_strict().unwrap().raw()).abs() <= 1);
        assert!((x.exp2().unwrap().raw() - x.exp2_strict().unwrap().raw()).abs() <= 1);
        assert!((x.powi(3).unwrap().raw() - x.powi_strict(3).unwrap().raw()).abs() <= 1);
    }
    for raw in [1, 250_000, 1_000_000, 2_000_000, 99_999_999] {
        let x = Number::from_raw(raw);
        assert!((x.ln().unwrap().raw() - x.ln_strict().unwrap().raw()).abs() <= 1);
        assert!((x.log2().unwrap().raw() - x.log2_strict().unwrap().raw()).abs() <= 1);
        assert!((x.log10().unwrap().raw() - x.log10_strict().unwrap().raw()).abs() <= 1);
        let base = Number::from_raw(2_000_000);
        assert!((x.log(base).unwrap().raw() - x.log_strict(base).unwrap().raw()).abs() <= 1);
    }
}

#[test]
fn arbitrary_denominators_and_large_inputs_do_not_use_f64() {
    let one = FixedU64::<{ u32::MAX }>::from_raw(u64::from(u32::MAX));
    assert_eq!(one.exp_strict().unwrap().raw(), 11_674_931_552);
    assert_eq!(
        FixedU64::<1>::from_raw(u64::MAX)
            .log2_strict()
            .unwrap()
            .raw(),
        64
    );
    assert_eq!(FixedI64::<3>::from_raw(1).exp_strict().unwrap().raw(), 4);
}
