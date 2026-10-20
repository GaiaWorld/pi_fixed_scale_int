use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64};

pub fn rounded_mul_half_ties() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(1), FixedU64::<10>::from_raw(5));
    assert_eq!(lhs.checked_mul(rhs).expect("fits").raw(), 0);
    assert_eq!(lhs.rounded_mul(rhs).expect("fits").raw(), 1);
    assert_eq!((lhs.raw(), rhs.raw()), (1, 5));
    for (left, expected) in [(1, 1), (-1, -1)] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(5));
        assert_eq!(lhs.checked_mul(rhs).expect("fits").raw(), 0);
        assert_eq!(lhs.rounded_mul(rhs).expect("fits").raw(), expected);
        assert_eq!((lhs.raw(), rhs.raw()), (left, 5));
    }
}

pub fn rounded_div_half_ties() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(1), FixedU64::<10>::from_raw(4));
    assert_eq!(lhs.checked_div(rhs).expect("fits").raw(), 2);
    assert_eq!(lhs.rounded_div(rhs).expect("fits").raw(), 3);
    assert_eq!((lhs.raw(), rhs.raw()), (1, 4));
    for (left, truncated, rounded) in [(1, 2, 3), (-1, -2, -3)] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(4));
        assert_eq!(lhs.checked_div(rhs).expect("fits").raw(), truncated);
        assert_eq!(lhs.rounded_div(rhs).expect("fits").raw(), rounded);
        assert_eq!((lhs.raw(), rhs.raw()), (left, 4));
    }
}

pub fn rounded_mul_either_side_of_half() {
    for (right, rounded) in [(4, 0), (6, 1)] {
        let (lhs, rhs) = (FixedU64::<10>::from_raw(1), FixedU64::<10>::from_raw(right));
        assert_eq!(lhs.checked_mul(rhs).expect("fits").raw(), 0);
        assert_eq!(lhs.rounded_mul(rhs).expect("fits").raw(), rounded);
        assert_eq!((lhs.raw(), rhs.raw()), (1, right));
    }
    for (left, right, rounded) in [(1, 4, 0), (-1, 4, 0), (1, 6, 1), (-1, 6, -1)] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(right));
        assert_eq!(lhs.checked_mul(rhs).expect("fits").raw(), 0);
        assert_eq!(lhs.rounded_mul(rhs).expect("fits").raw(), rounded);
        assert_eq!((lhs.raw(), rhs.raw()), (left, right));
    }
}

pub fn rounded_div_either_side_of_half() {
    for (right, rounded) in [(7, 1), (6, 2)] {
        let (lhs, rhs) = (FixedU64::<10>::from_raw(1), FixedU64::<10>::from_raw(right));
        assert_eq!(lhs.checked_div(rhs).expect("fits").raw(), 1);
        assert_eq!(lhs.rounded_div(rhs).expect("fits").raw(), rounded);
        assert_eq!((lhs.raw(), rhs.raw()), (1, right));
    }
    for (left, right, truncated, rounded) in [
        (1, 7, 1, 1), (-1, 7, -1, -1), (1, 6, 1, 2), (-1, 6, -1, -2),
    ] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(right));
        assert_eq!(lhs.checked_div(rhs).expect("fits").raw(), truncated);
        assert_eq!(lhs.rounded_div(rhs).expect("fits").raw(), rounded);
        assert_eq!((lhs.raw(), rhs.raw()), (left, right));
    }
}

pub fn rounded_div_negative_divisor() {
    for (left, rounded) in [(1, -3), (-1, 3)] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(-4));
        assert_eq!(lhs.rounded_div(rhs).expect("fits").raw(), rounded);
        assert_eq!((lhs.raw(), rhs.raw()), (left, -4));
    }
}

pub fn rounded_exact_results() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(2), FixedU64::<10>::from_raw(5));
    assert_eq!(lhs.rounded_mul(rhs).expect("fits").raw(), 1);
    assert_eq!(lhs.rounded_div(rhs).expect("fits").raw(), 4);
    assert_eq!((lhs.raw(), rhs.raw()), (2, 5));
    let (lhs, rhs) = (FixedI64::<10>::from_raw(-2), FixedI64::<10>::from_raw(5));
    assert_eq!(lhs.rounded_mul(rhs).expect("fits").raw(), -1);
    assert_eq!(lhs.rounded_div(rhs).expect("fits").raw(), -4);
    assert_eq!((lhs.raw(), rhs.raw()), (-2, 5));
}

pub fn rounded_division_by_zero() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(1), FixedU64::<10>::from_raw(0));
    assert_eq!(lhs.rounded_div(rhs).map(|v| v.raw()), Err(ArithmeticError::DivisionByZero));
    assert_eq!((lhs.raw(), rhs.raw()), (1, 0));
    let (lhs, rhs) = (FixedI64::<10>::from_raw(-1), FixedI64::<10>::from_raw(0));
    assert_eq!(lhs.rounded_div(rhs).map(|v| v.raw()), Err(ArithmeticError::DivisionByZero));
    assert_eq!((lhs.raw(), rhs.raw()), (-1, 0));
}

pub fn rounded_arithmetic_overflows() {
    let (lhs, rhs) = (FixedU64::<10>::from_raw(u64::MAX), FixedU64::<10>::from_raw(11));
    assert_eq!(lhs.rounded_mul(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((lhs.raw(), rhs.raw()), (u64::MAX, 11));
    let (lhs, rhs) = (FixedU64::<10>::from_raw(u64::MAX), FixedU64::<10>::from_raw(1));
    assert_eq!(lhs.rounded_div(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((lhs.raw(), rhs.raw()), (u64::MAX, 1));
    let (lhs, rhs) = (FixedI64::<10>::from_raw(i64::MIN), FixedI64::<10>::from_raw(-10));
    assert_eq!(lhs.rounded_mul(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((lhs.raw(), rhs.raw()), (i64::MIN, -10));
}

pub fn rounded_mul_crosses_final_bound() {
    let (lhs, rhs) = (
        FixedU64::<10>::from_raw(15_372_286_728_091_293_013),
        FixedU64::<10>::from_raw(12),
    );
    assert_eq!(lhs.checked_mul(rhs).expect("truncation fits").raw(), u64::MAX);
    assert_eq!(lhs.rounded_mul(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((lhs.raw(), rhs.raw()), (15_372_286_728_091_293_013, 12));

    for (left, right, truncated) in [
        (6_148_914_691_236_517_205, 15, i64::MAX),
        (-8_384_883_669_867_978_008, 11, i64::MIN),
    ] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(right));
        assert_eq!(lhs.checked_mul(rhs).expect("truncation fits").raw(), truncated);
        assert_eq!(lhs.rounded_mul(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
        assert_eq!((lhs.raw(), rhs.raw()), (left, right));
    }
}

pub fn rounded_div_crosses_final_bound() {
    let (lhs, rhs) = (
        FixedU64::<10>::from_raw(12_912_720_851_596_686_131),
        FixedU64::<10>::from_raw(7),
    );
    assert_eq!(lhs.checked_div(rhs).expect("truncation fits").raw(), u64::MAX);
    assert_eq!(lhs.rounded_div(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!((lhs.raw(), rhs.raw()), (12_912_720_851_596_686_131, 7));

    for (left, right, truncated) in [
        (3_689_348_814_741_910_323, 4, i64::MAX),
        (-6_456_360_425_798_343_066, 7, i64::MIN),
    ] {
        let (lhs, rhs) = (FixedI64::<10>::from_raw(left), FixedI64::<10>::from_raw(right));
        assert_eq!(lhs.checked_div(rhs).expect("truncation fits").raw(), truncated);
        assert_eq!(lhs.rounded_div(rhs).map(|v| v.raw()), Err(ArithmeticError::Overflow));
        assert_eq!((lhs.raw(), rhs.raw()), (left, right));
    }
}
