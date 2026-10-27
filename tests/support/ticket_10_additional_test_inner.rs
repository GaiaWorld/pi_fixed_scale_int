use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};
pub fn exact_binary_half_ties() {
    for sign in [-1, 1] {
        let x = FixedI64::<1>::from_raw(sign * 2);
        assert_eq!(x.powi_strict(-1).unwrap().raw(), sign);
        assert_eq!(x.raw(), sign * 2);
        let x = FixedI64::<10>::from_raw(sign * 5);
        assert_eq!(x.powi_strict(3).unwrap().raw(), sign);
        assert_eq!(x.raw(), sign * 5);
    }
    let x = FixedU64::<1>::from_raw(2);
    assert_eq!(x.powi_strict(-1).unwrap().raw(), 1);
    assert_eq!(x.raw(), 2);
    let x = FixedU64::<10>::from_raw(5);
    assert_eq!(x.powi_strict(3).unwrap().raw(), 1);
    assert_eq!(x.raw(), 5);
}
pub fn power_overflow() {
    for raw in [i64::MIN, i64::MAX] {
        let x = FixedI64::<1>::from_raw(raw);
        assert_eq!(x.powi_strict(2), Err(MathError::Overflow));
        assert_eq!(x.raw(), raw);
    }
    let x = FixedU64::<1>::from_raw(u64::MAX);
    assert_eq!(x.powi_strict(2), Err(MathError::Overflow));
    assert_eq!(x.raw(), u64::MAX);
}
