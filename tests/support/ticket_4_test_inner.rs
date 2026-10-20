use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64};

pub fn checked_mul_scales() {
    let (lhs, rhs) = (FixedU64::<100>::from_raw(150), FixedU64::<100>::from_raw(50));
    assert_eq!(lhs.checked_mul(rhs).expect("fits").raw(), 75);
    assert_eq!((lhs.raw(), rhs.raw()), (150, 50));
    let (lhs, rhs) = (FixedI64::<100>::from_raw(-150), FixedI64::<100>::from_raw(50));
    assert_eq!(lhs.checked_mul(rhs).expect("fits").raw(), -75);
    assert_eq!((lhs.raw(), rhs.raw()), (-150, 50));
}

pub fn checked_div_scales() {
    let (lhs, rhs) = (FixedU64::<100>::from_raw(150), FixedU64::<100>::from_raw(50));
    assert_eq!(lhs.checked_div(rhs).expect("fits").raw(), 300);
    assert_eq!((lhs.raw(), rhs.raw()), (150, 50));
    let (lhs, rhs) = (FixedI64::<100>::from_raw(-150), FixedI64::<100>::from_raw(50));
    assert_eq!(lhs.checked_div(rhs).expect("fits").raw(), -300);
    assert_eq!((lhs.raw(), rhs.raw()), (-150, 50));
}

pub fn checked_div_negative_divisor() {
    for (left, expected) in [(150, -300), (-150, 300)] {
        let lhs = FixedI64::<100>::from_raw(left);
        let rhs = FixedI64::<100>::from_raw(-50);
        assert_eq!(lhs.checked_div(rhs).expect("fits").raw(), expected);
        assert_eq!((lhs.raw(), rhs.raw()), (left, -50));
    }
}

pub fn checked_mul_truncates() {
    let (lhs, rhs) = (FixedI64::<10>::from_raw(-1), FixedI64::<10>::from_raw(5));
    assert_eq!(lhs.checked_mul(rhs).expect("fits").raw(), 0);
    assert_eq!((lhs.raw(), rhs.raw()), (-1, 5));
}

pub fn checked_div_truncates() {
    let (lhs, rhs) = (FixedI64::<10>::from_raw(-1), FixedI64::<10>::from_raw(4));
    assert_eq!(lhs.checked_div(rhs).expect("fits").raw(), -2);
    assert_eq!((lhs.raw(), rhs.raw()), (-1, 4));
}

pub fn checked_mul_overflows() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(u64::MAX), FixedU64::<10>::from_raw(11));
    assert_eq!(lhs.checked_mul(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((lhs.raw(), rhs.raw()), (u64::MAX, 11));
    for left in [i64::MAX, i64::MIN] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(11));
        assert_eq!(lhs.checked_mul(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
        assert_eq!((lhs.raw(), rhs.raw()), (left, 11));
    }
}

pub fn checked_div_overflows() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(u64::MAX), FixedU64::<10>::from_raw(1));
    assert_eq!(lhs.checked_div(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((lhs.raw(), rhs.raw()), (u64::MAX, 1));
    for left in [i64::MAX, i64::MIN] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(1));
        assert_eq!(lhs.checked_div(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
        assert_eq!((lhs.raw(), rhs.raw()), (left, 1));
    }
}

pub fn saturating_mul_fits() {
    let (lhs, rhs) = (FixedU64::<100>::from_raw(150), FixedU64::<100>::from_raw(50));
    assert_eq!(lhs.saturating_mul(rhs).raw(), 75);
    assert_eq!((lhs.raw(), rhs.raw()), (150, 50));
}

pub fn saturating_div_fits() {
    let (lhs, rhs) = (FixedU64::<100>::from_raw(150), FixedU64::<100>::from_raw(50));
    assert_eq!(lhs.saturating_div(rhs).expect("nonzero divisor").raw(), 300);
    assert_eq!((lhs.raw(), rhs.raw()), (150, 50));
}

pub fn saturating_div_negative_divisor() {
    for (left, expected) in [(150, -300), (-150, 300)] {
        let lhs = FixedI64::<100>::from_raw(left);
        let rhs = FixedI64::<100>::from_raw(-50);
        assert_eq!(lhs.saturating_div(rhs).expect("nonzero divisor").raw(), expected);
        assert_eq!((lhs.raw(), rhs.raw()), (left, -50));
    }
}

pub fn saturating_mul_clamps() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(u64::MAX), FixedU64::<10>::from_raw(11));
    assert_eq!(lhs.saturating_mul(rhs).raw(), u64::MAX);
    assert_eq!((lhs.raw(), rhs.raw()), (u64::MAX, 11));
    for left in [i64::MAX, i64::MIN] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(11));
        assert_eq!(lhs.saturating_mul(rhs).raw(), left);
        assert_eq!((lhs.raw(), rhs.raw()), (left, 11));
    }
}

pub fn saturating_div_clamps() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(u64::MAX), FixedU64::<10>::from_raw(1));
    assert_eq!(lhs.saturating_div(rhs).expect("nonzero divisor").raw(), u64::MAX);
    assert_eq!((lhs.raw(), rhs.raw()), (u64::MAX, 1));
    for left in [i64::MAX, i64::MIN] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(1));
        assert_eq!(lhs.saturating_div(rhs).expect("nonzero divisor").raw(), left);
        assert_eq!((lhs.raw(), rhs.raw()), (left, 1));
    }
}

pub fn checked_division_by_zero() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(u64::MAX), FixedU64::<10>::from_raw(0));
    assert_eq!(lhs.checked_div(rhs).map(|v| v.raw()), Err(ArithmeticError::DivisionByZero));
    assert_eq!((lhs.raw(), rhs.raw()), (u64::MAX, 0));
    let (lhs, rhs) = (FixedI64::<10>::from_raw(-15), FixedI64::<10>::from_raw(0));
    assert_eq!(lhs.checked_div(rhs).map(|v| v.raw()), Err(ArithmeticError::DivisionByZero));
    assert_eq!((lhs.raw(), rhs.raw()), (-15, 0));
}

pub fn saturating_division_by_zero() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(u64::MAX), FixedU64::<10>::from_raw(0));
    assert_eq!(lhs.saturating_div(rhs).map(|v| v.raw()), Err(ArithmeticError::DivisionByZero));
    assert_eq!((lhs.raw(), rhs.raw()), (u64::MAX, 0));
    let (lhs, rhs) = (FixedI64::<10>::from_raw(-15), FixedI64::<10>::from_raw(0));
    assert_eq!(lhs.saturating_div(rhs).map(|v| v.raw()), Err(ArithmeticError::DivisionByZero));
    assert_eq!((lhs.raw(), rhs.raw()), (-15, 0));
}
