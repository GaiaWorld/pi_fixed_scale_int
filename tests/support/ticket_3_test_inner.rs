use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64};

pub fn checked_add_returns_exact_raw_sum() {
    let left = FixedU64::<100>::from_raw(150);
    let right = FixedU64::<100>::from_raw(50);
    assert_eq!(left.checked_add(right).expect("sum fits").raw(), 200);
    assert_eq!((left.raw(), right.raw()), (150, 50));

    let left = FixedI64::<100>::from_raw(-150);
    let right = FixedI64::<100>::from_raw(50);
    assert_eq!(left.checked_add(right).expect("sum fits").raw(), -100);
    assert_eq!((left.raw(), right.raw()), (-150, 50));
}

pub fn checked_sub_returns_exact_raw_difference() {
    let left = FixedU64::<100>::from_raw(150);
    let right = FixedU64::<100>::from_raw(50);
    assert_eq!(left.checked_sub(right).expect("difference fits").raw(), 100);
    assert_eq!((left.raw(), right.raw()), (150, 50));

    let left = FixedI64::<100>::from_raw(-150);
    let right = FixedI64::<100>::from_raw(50);
    assert_eq!(left.checked_sub(right).expect("difference fits").raw(), -200);
    assert_eq!((left.raw(), right.raw()), (-150, 50));

    let left = FixedI64::<100>::from_raw(150);
    let right = FixedI64::<100>::from_raw(-50);
    assert_eq!(left.checked_sub(right).expect("difference fits").raw(), 200);
    assert_eq!((left.raw(), right.raw()), (150, -50));
}

pub fn checked_add_above_max_overflows() {
    let left = FixedU64::<10>::from_raw(u64::MAX);
    let right = FixedU64::<10>::from_raw(1);
    assert_eq!(left.checked_add(right).map(|value| value.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((left.raw(), right.raw()), (u64::MAX, 1));

    let left = FixedI64::<10>::from_raw(i64::MAX);
    let right = FixedI64::<10>::from_raw(1);
    assert_eq!(left.checked_add(right).map(|value| value.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((left.raw(), right.raw()), (i64::MAX, 1));
}

pub fn checked_add_below_signed_min_overflows() {
    let left = FixedI64::<10>::from_raw(i64::MIN);
    let right = FixedI64::<10>::from_raw(-1);
    assert_eq!(left.checked_add(right).map(|value| value.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((left.raw(), right.raw()), (i64::MIN, -1));
}

pub fn checked_unsigned_sub_below_zero_overflows() {
    for (lhs, rhs) in [(0, 1), (2, 5)] {
        let left = FixedU64::<10>::from_raw(lhs);
        let right = FixedU64::<10>::from_raw(rhs);
        assert_eq!(left.checked_sub(right).map(|value| value.raw()), Err(ArithmeticError::Overflow));
        assert_eq!((left.raw(), right.raw()), (lhs, rhs));
    }
}

pub fn checked_signed_sub_outside_bounds_overflows() {
    for (lhs, rhs) in [(i64::MIN, 1), (i64::MAX, -1)] {
        let left = FixedI64::<10>::from_raw(lhs);
        let right = FixedI64::<10>::from_raw(rhs);
        assert_eq!(left.checked_sub(right).map(|value| value.raw()), Err(ArithmeticError::Overflow));
        assert_eq!((left.raw(), right.raw()), (lhs, rhs));
    }
}

pub fn saturating_add_without_overflow_returns_exact_sum() {
    let left = FixedU64::<100>::from_raw(150);
    let right = FixedU64::<100>::from_raw(50);
    assert_eq!(left.saturating_add(right).raw(), 200);
    assert_eq!((left.raw(), right.raw()), (150, 50));

    let left = FixedI64::<100>::from_raw(-150);
    let right = FixedI64::<100>::from_raw(50);
    assert_eq!(left.saturating_add(right).raw(), -100);
    assert_eq!((left.raw(), right.raw()), (-150, 50));
}

pub fn saturating_sub_without_overflow_returns_exact_difference() {
    let left = FixedU64::<100>::from_raw(150);
    let right = FixedU64::<100>::from_raw(50);
    assert_eq!(left.saturating_sub(right).raw(), 100);
    assert_eq!((left.raw(), right.raw()), (150, 50));

    let left = FixedI64::<100>::from_raw(-150);
    let right = FixedI64::<100>::from_raw(50);
    assert_eq!(left.saturating_sub(right).raw(), -200);
    assert_eq!((left.raw(), right.raw()), (-150, 50));

    let left = FixedI64::<100>::from_raw(150);
    let right = FixedI64::<100>::from_raw(-50);
    assert_eq!(left.saturating_sub(right).raw(), 200);
    assert_eq!((left.raw(), right.raw()), (150, -50));
}

pub fn saturating_add_above_max_clamps_to_max() {
    let left = FixedU64::<10>::from_raw(u64::MAX);
    let right = FixedU64::<10>::from_raw(1);
    assert_eq!(left.saturating_add(right).raw(), u64::MAX);
    assert_eq!((left.raw(), right.raw()), (u64::MAX, 1));

    let left = FixedI64::<10>::from_raw(i64::MAX);
    let right = FixedI64::<10>::from_raw(1);
    assert_eq!(left.saturating_add(right).raw(), i64::MAX);
    assert_eq!((left.raw(), right.raw()), (i64::MAX, 1));
}

pub fn saturating_add_below_signed_min_clamps_to_min() {
    let left = FixedI64::<10>::from_raw(i64::MIN);
    let right = FixedI64::<10>::from_raw(-1);
    assert_eq!(left.saturating_add(right).raw(), i64::MIN);
    assert_eq!((left.raw(), right.raw()), (i64::MIN, -1));
}

pub fn saturating_unsigned_sub_below_zero_clamps_to_zero() {
    for (lhs, rhs) in [(0, 1), (2, 5)] {
        let left = FixedU64::<10>::from_raw(lhs);
        let right = FixedU64::<10>::from_raw(rhs);
        assert_eq!(left.saturating_sub(right).raw(), 0);
        assert_eq!((left.raw(), right.raw()), (lhs, rhs));
    }
}

pub fn saturating_signed_sub_outside_bounds_clamps() {
    for (lhs, rhs, expected) in [(i64::MIN, 1, i64::MIN), (i64::MAX, -1, i64::MAX)] {
        let left = FixedI64::<10>::from_raw(lhs);
        let right = FixedI64::<10>::from_raw(rhs);
        assert_eq!(left.saturating_sub(right).raw(), expected);
        assert_eq!((left.raw(), right.raw()), (lhs, rhs));
    }
}
