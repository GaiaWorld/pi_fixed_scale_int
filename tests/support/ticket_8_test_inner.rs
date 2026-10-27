use pi_fixed_scale_int::{ArithmeticError as E, FixedI64 as I, FixedU64 as U};

pub fn mixed_fixed_mul() {
    let u = U::<100>::from_raw(150);
    let i = I::<100>::from_raw(150);
    let ri = I::<10>::from_raw(5);
    let ru = U::<10>::from_raw(5);
    let a: U<100> = u.checked_mul(ri).unwrap();
    let b: I<100> = i.checked_mul(ru).unwrap();
    assert_eq!((a.raw(), b.raw()), (75, 75));
    assert_eq!(u.checked_mul_with(ri).unwrap().raw(), 75);
    assert_eq!(i.checked_mul_with(ru).unwrap().raw(), 75);
    assert_eq!((u.raw(), i.raw(), ri.raw(), ru.raw()), (150, 150, 5, 5));
}

pub fn primitive_mul() {
    let u = U::<100>::from_raw(150);
    let i = I::<100>::from_raw(150);
    macro_rules! check { ($($t:ty),*) => {$(let a:U<100>=u.checked_mul(100 as $t).unwrap(); let b:I<100>=i.checked_mul(100 as $t).unwrap(); assert_eq!((a.raw(),b.raw()),(15000,15000));)*}; }
    check!(u8, u16, u32, u64, i8, i16, i32, i64);
    assert_eq!((u.raw(), i.raw()), (150, 150));
}

pub fn near_zero_mul_checked() {
    for (r, magnitude) in [(49, 0), (50, 1), (51, 1)] {
        for sign in [-1i64, 1] {
            let lhs = I::<10>::from_raw(sign);
            let rhs = U::<100>::from_raw(r);
            let expected = 0;
            assert_eq!(lhs.checked_mul(rhs).unwrap().raw(), expected);
            assert_eq!(lhs.checked_mul_with(rhs).unwrap().raw(), expected);
            let neg = I::<100>::from_raw(-(r as i64));
            assert_eq!(lhs.checked_mul(neg).unwrap().raw(), -expected);
            assert_eq!(lhs.checked_mul_with(neg).unwrap().raw(), -expected);
            assert_eq!((lhs.raw(), rhs.raw(), neg.raw()), (sign, r, -(r as i64)));
        }
        for sign in [-1i64, 1] {
            let lhs = U::<10>::from_raw(1);
            let rhs = I::<100>::from_raw(sign * r as i64);
            let expected: Result<U<10>, E> = {
                let _ = magnitude;
                Ok(U::from_raw(0))
            };
            assert_eq!(lhs.checked_mul(rhs), expected);
            assert_eq!(lhs.checked_mul_with(rhs), expected);
            assert_eq!((lhs.raw(), rhs.raw()), (1, sign * r as i64));
        }
    }
}

pub fn near_zero_mul_rounded() {
    for (r, magnitude) in [(49, 0), (50, 1), (51, 1)] {
        for sign in [-1i64, 1] {
            let lhs = I::<10>::from_raw(sign);
            let rhs = U::<100>::from_raw(r);
            let expected = sign * magnitude;
            assert_eq!(lhs.rounded_mul(rhs).unwrap().raw(), expected);
            assert_eq!(lhs.rounded_mul_with(rhs).unwrap().raw(), expected);
            let neg = I::<100>::from_raw(-(r as i64));
            assert_eq!(lhs.rounded_mul(neg).unwrap().raw(), -expected);
            assert_eq!(lhs.rounded_mul_with(neg).unwrap().raw(), -expected);
            assert_eq!((lhs.raw(), rhs.raw(), neg.raw()), (sign, r, -(r as i64)));
        }
        for sign in [-1i64, 1] {
            let lhs = U::<10>::from_raw(1);
            let rhs = I::<100>::from_raw(sign * r as i64);
            let expected: Result<U<10>, E> = if sign < 0 && magnitude != 0 {
                Err(E::Overflow)
            } else {
                Ok(U::from_raw(magnitude as u64))
            };
            assert_eq!(lhs.rounded_mul(rhs), expected);
            assert_eq!(lhs.rounded_mul_with(rhs), expected);
            assert_eq!((lhs.raw(), rhs.raw()), (1, sign * r as i64));
        }
    }
}

pub fn negative_unsigned_mul_checked() {
    let lhs = U::<100>::from_raw(150);
    let rhs = I::<10>::from_raw(-5);
    macro_rules! check {
        ($r:expr) => {
            assert_eq!(lhs.checked_mul($r), Err(E::Overflow));
        };
    }
    check!(rhs);
    check!(-2i8);
    assert_eq!((lhs.raw(), rhs.raw()), (150, -5));
}

pub fn negative_unsigned_mul_rounded() {
    let lhs = U::<100>::from_raw(150);
    let rhs = I::<10>::from_raw(-5);
    macro_rules! check {
        ($r:expr) => {
            assert_eq!(lhs.rounded_mul($r), Err(E::Overflow));
        };
    }
    check!(rhs);
    check!(-2i8);
    assert_eq!((lhs.raw(), rhs.raw()), (150, -5));
}

pub fn negative_unsigned_mul_saturating() {
    let lhs = U::<100>::from_raw(150);
    let rhs = I::<10>::from_raw(-5);
    macro_rules! check {
        ($r:expr) => {
            assert_eq!(lhs.saturating_mul($r).raw(), 0);
        };
    }
    check!(rhs);
    check!(-2i8);
    assert_eq!((lhs.raw(), rhs.raw()), (150, -5));
}

pub fn range_mul_checked() {
    macro_rules! check {
        ($lhs:expr) => {{
            let lhs = $lhs;
            let rhs = 2u8;
            let before = lhs.raw();
            assert_eq!(lhs.checked_mul(rhs), Err(E::Overflow));
            assert_eq!(lhs.checked_mul_with(rhs), Err(E::Overflow));
            assert_eq!(lhs.raw(), before);
            assert_eq!(rhs, 2u8);
        }};
    }
    check!(U::<10>::from_raw(u64::MAX));
    check!(I::<10>::from_raw(i64::MAX));
    check!(I::<10>::from_raw(i64::MIN));
    let lhs = I::<10>::from_raw(i64::MIN);
    assert_eq!(lhs.checked_mul(-1i8), Err(E::Overflow));
    assert_eq!(lhs.raw(), i64::MIN);
}

pub fn wide_mul_checked() {
    macro_rules! check {
        ($lhs:expr,$rhs:expr,$expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            assert_eq!(lhs.checked_mul(rhs).unwrap().raw(), $expected);
            assert_eq!(lhs.raw(), $expected);
            assert_eq!(rhs.raw() as u64, u64::from(u32::MAX));
        }};
    }
    check!(
        U::<100>::from_raw(u64::MAX),
        I::<{ u32::MAX }>::from_raw(i64::from(u32::MAX)),
        u64::MAX
    );
    check!(
        I::<100>::from_raw(i64::MIN),
        U::<{ u32::MAX }>::from_raw(u64::from(u32::MAX)),
        i64::MIN
    );
}

pub fn range_mul_rounded() {
    macro_rules! check {
        ($lhs:expr) => {{
            let lhs = $lhs;
            let rhs = 2u8;
            let before = lhs.raw();
            assert_eq!(lhs.rounded_mul(rhs), Err(E::Overflow));
            assert_eq!(lhs.rounded_mul_with(rhs), Err(E::Overflow));
            assert_eq!(lhs.raw(), before);
            assert_eq!(rhs, 2u8);
        }};
    }
    check!(U::<10>::from_raw(u64::MAX));
    check!(I::<10>::from_raw(i64::MAX));
    check!(I::<10>::from_raw(i64::MIN));
    let lhs = I::<10>::from_raw(i64::MIN);
    assert_eq!(lhs.rounded_mul(-1i8), Err(E::Overflow));
    assert_eq!(lhs.raw(), i64::MIN);
}

pub fn wide_mul_rounded() {
    macro_rules! check {
        ($lhs:expr,$rhs:expr,$expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            assert_eq!(lhs.rounded_mul(rhs).unwrap().raw(), $expected);
            assert_eq!(lhs.raw(), $expected);
            assert_eq!(rhs.raw() as u64, u64::from(u32::MAX));
        }};
    }
    check!(
        U::<100>::from_raw(u64::MAX),
        I::<{ u32::MAX }>::from_raw(i64::from(u32::MAX)),
        u64::MAX
    );
    check!(
        I::<100>::from_raw(i64::MIN),
        U::<{ u32::MAX }>::from_raw(u64::from(u32::MAX)),
        i64::MIN
    );
}

pub fn range_mul_saturating() {
    macro_rules! check {
        ($lhs:expr,$bound:expr) => {{
            let lhs = $lhs;
            let rhs = 2u8;
            let before = lhs.raw();
            assert_eq!(lhs.saturating_mul(rhs).raw(), $bound);
            assert_eq!(lhs.saturating_mul_with(rhs).raw(), $bound);
            assert_eq!(lhs.raw(), before);
            assert_eq!(rhs, 2u8);
        }};
    }
    check!(U::<10>::from_raw(u64::MAX), u64::MAX);
    check!(I::<10>::from_raw(i64::MAX), i64::MAX);
    check!(I::<10>::from_raw(i64::MIN), i64::MIN);
    let lhs = I::<10>::from_raw(i64::MIN);
    assert_eq!(lhs.saturating_mul(-1i8).raw(), i64::MAX);
    assert_eq!(lhs.raw(), i64::MIN);
}

pub fn wide_mul_saturating() {
    macro_rules! check {
        ($lhs:expr,$rhs:expr,$expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            assert_eq!(lhs.saturating_mul(rhs).raw(), $expected);
            assert_eq!(lhs.raw(), $expected);
            assert_eq!(rhs.raw() as u64, u64::from(u32::MAX));
        }};
    }
    check!(
        U::<100>::from_raw(u64::MAX),
        I::<{ u32::MAX }>::from_raw(i64::from(u32::MAX)),
        u64::MAX
    );
    check!(
        I::<100>::from_raw(i64::MIN),
        U::<{ u32::MAX }>::from_raw(u64::from(u32::MAX)),
        i64::MIN
    );
}

pub fn negative_primitive_mul() {
    let lhs = I::<100>::from_raw(150);
    let result: I<100> = lhs.checked_mul(-2i8).unwrap();
    assert_eq!(result.raw(), -300);
    assert_eq!(lhs.raw(), 150);
}

pub fn rounding_crosses_bound_mul() {
    // 精确乘积：1190112520884487201*31=2*MAX_U+1；6148914691236517205*3=2*MAX_I+1；67280421310721*274177=2*abs(MIN_I)+1。
    macro_rules! check {
        ($lhs:expr,$rhs:expr,$expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            let before = (lhs.raw(), rhs.raw());
            assert_eq!(lhs.checked_mul(rhs).unwrap().raw(), $expected);
            assert_eq!(lhs.checked_mul_with(rhs).unwrap().raw(), $expected);
            assert_eq!(lhs.rounded_mul(rhs), Err(E::Overflow));
            assert_eq!(lhs.rounded_mul_with(rhs), Err(E::Overflow));
            assert_eq!((lhs.raw(), rhs.raw()), before);
        }};
    }
    check!(
        U::<10>::from_raw(1190112520884487201),
        I::<2>::from_raw(31),
        u64::MAX
    );
    check!(
        I::<10>::from_raw(6148914691236517205),
        U::<2>::from_raw(3),
        i64::MAX
    );
    check!(
        I::<10>::from_raw(-67280421310721),
        U::<2>::from_raw(274177),
        i64::MIN
    );
}

pub fn mixed_fixed_div() {
    let u = U::<100>::from_raw(150);
    let i = I::<100>::from_raw(150);
    let ri = I::<10>::from_raw(5);
    let ru = U::<10>::from_raw(5);
    let a: U<100> = u.checked_div(ri).unwrap();
    let b: I<100> = i.checked_div(ru).unwrap();
    assert_eq!((a.raw(), b.raw()), (300, 300));
    assert_eq!(u.checked_div_with(ri).unwrap().raw(), 300);
    assert_eq!(i.checked_div_with(ru).unwrap().raw(), 300);
    assert_eq!((u.raw(), i.raw(), ri.raw(), ru.raw()), (150, 150, 5, 5));
}

pub fn primitive_div() {
    let u = U::<100>::from_raw(150);
    let i = I::<100>::from_raw(150);
    macro_rules! check { ($($t:ty),*) => {$(let a:U<100>=u.checked_div(2 as $t).unwrap(); let b:I<100>=i.checked_div(2 as $t).unwrap(); assert_eq!((a.raw(),b.raw()),(75,75));)*}; }
    check!(u8, u16, u32, u64, i8, i16, i32, i64);
    assert_eq!((u.raw(), i.raw()), (150, 150));
}

pub fn near_zero_div_checked() {
    for (r, magnitude) in [(201, 0), (200, 1), (199, 1)] {
        for sign in [-1i64, 1] {
            let lhs = I::<10>::from_raw(sign);
            let rhs = U::<100>::from_raw(r);
            let expected = 0;
            assert_eq!(lhs.checked_div(rhs).unwrap().raw(), expected);
            assert_eq!(lhs.checked_div_with(rhs).unwrap().raw(), expected);
            let neg = I::<100>::from_raw(-(r as i64));
            assert_eq!(lhs.checked_div(neg).unwrap().raw(), -expected);
            assert_eq!(lhs.checked_div_with(neg).unwrap().raw(), -expected);
            assert_eq!((lhs.raw(), rhs.raw(), neg.raw()), (sign, r, -(r as i64)));
        }
        for sign in [-1i64, 1] {
            let lhs = U::<10>::from_raw(1);
            let rhs = I::<100>::from_raw(sign * r as i64);
            let expected: Result<U<10>, E> = {
                let _ = magnitude;
                Ok(U::from_raw(0))
            };
            assert_eq!(lhs.checked_div(rhs), expected);
            assert_eq!(lhs.checked_div_with(rhs), expected);
            assert_eq!((lhs.raw(), rhs.raw()), (1, sign * r as i64));
        }
    }
}

pub fn near_zero_div_rounded() {
    for (r, magnitude) in [(201, 0), (200, 1), (199, 1)] {
        for sign in [-1i64, 1] {
            let lhs = I::<10>::from_raw(sign);
            let rhs = U::<100>::from_raw(r);
            let expected = sign * magnitude;
            assert_eq!(lhs.rounded_div(rhs).unwrap().raw(), expected);
            assert_eq!(lhs.rounded_div_with(rhs).unwrap().raw(), expected);
            let neg = I::<100>::from_raw(-(r as i64));
            assert_eq!(lhs.rounded_div(neg).unwrap().raw(), -expected);
            assert_eq!(lhs.rounded_div_with(neg).unwrap().raw(), -expected);
            assert_eq!((lhs.raw(), rhs.raw(), neg.raw()), (sign, r, -(r as i64)));
        }
        for sign in [-1i64, 1] {
            let lhs = U::<10>::from_raw(1);
            let rhs = I::<100>::from_raw(sign * r as i64);
            let expected: Result<U<10>, E> = if sign < 0 && magnitude != 0 {
                Err(E::Overflow)
            } else {
                Ok(U::from_raw(magnitude as u64))
            };
            assert_eq!(lhs.rounded_div(rhs), expected);
            assert_eq!(lhs.rounded_div_with(rhs), expected);
            assert_eq!((lhs.raw(), rhs.raw()), (1, sign * r as i64));
        }
    }
}

pub fn negative_unsigned_div_checked() {
    let lhs = U::<100>::from_raw(150);
    let rhs = I::<10>::from_raw(-5);
    macro_rules! check {
        ($r:expr) => {
            assert_eq!(lhs.checked_div($r), Err(E::Overflow));
        };
    }
    check!(rhs);
    check!(-2i8);
    assert_eq!((lhs.raw(), rhs.raw()), (150, -5));
}

pub fn negative_unsigned_div_rounded() {
    let lhs = U::<100>::from_raw(150);
    let rhs = I::<10>::from_raw(-5);
    macro_rules! check {
        ($r:expr) => {
            assert_eq!(lhs.rounded_div($r), Err(E::Overflow));
        };
    }
    check!(rhs);
    check!(-2i8);
    assert_eq!((lhs.raw(), rhs.raw()), (150, -5));
}

pub fn negative_unsigned_div_saturating() {
    let lhs = U::<100>::from_raw(150);
    let rhs = I::<10>::from_raw(-5);
    macro_rules! check {
        ($r:expr) => {
            assert_eq!(lhs.saturating_div($r).unwrap().raw(), 0);
        };
    }
    check!(rhs);
    check!(-2i8);
    assert_eq!((lhs.raw(), rhs.raw()), (150, -5));
}

pub fn range_div_checked() {
    macro_rules! check {
        ($lhs:expr,$half:expr,$bound:expr) => {{
            let lhs = $lhs;
            let half = $half;
            let before = lhs.raw();
            assert_eq!(lhs.checked_div(half), Err(E::Overflow));
            assert_eq!(lhs.checked_div_with(half), Err(E::Overflow));
            assert_eq!(lhs.raw(), before);
            assert_eq!(half.raw(), 1);
        }};
    }
    check!(U::<10>::from_raw(u64::MAX), I::<2>::from_raw(1), u64::MAX);
    check!(I::<10>::from_raw(i64::MAX), U::<2>::from_raw(1), i64::MAX);
    check!(I::<10>::from_raw(i64::MIN), U::<2>::from_raw(1), i64::MIN);
    let lhs = I::<10>::from_raw(i64::MIN);
    assert_eq!(lhs.checked_div(-1i8), Err(E::Overflow));
    assert_eq!(lhs.raw(), i64::MIN);
}

pub fn wide_div_checked() {
    macro_rules! check {
        ($lhs:expr,$rhs:expr,$expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            assert_eq!(lhs.checked_div(rhs).unwrap().raw(), $expected);
            assert_eq!(lhs.raw(), $expected);
            assert_eq!(rhs.raw() as u64, u64::from(u32::MAX));
        }};
    }
    check!(
        U::<100>::from_raw(u64::MAX),
        I::<{ u32::MAX }>::from_raw(i64::from(u32::MAX)),
        u64::MAX
    );
    check!(
        I::<100>::from_raw(i64::MIN),
        U::<{ u32::MAX }>::from_raw(u64::from(u32::MAX)),
        i64::MIN
    );
}

pub fn range_div_rounded() {
    macro_rules! check {
        ($lhs:expr,$half:expr,$bound:expr) => {{
            let lhs = $lhs;
            let half = $half;
            let before = lhs.raw();
            assert_eq!(lhs.rounded_div(half), Err(E::Overflow));
            assert_eq!(lhs.rounded_div_with(half), Err(E::Overflow));
            assert_eq!(lhs.raw(), before);
            assert_eq!(half.raw(), 1);
        }};
    }
    check!(U::<10>::from_raw(u64::MAX), I::<2>::from_raw(1), u64::MAX);
    check!(I::<10>::from_raw(i64::MAX), U::<2>::from_raw(1), i64::MAX);
    check!(I::<10>::from_raw(i64::MIN), U::<2>::from_raw(1), i64::MIN);
    let lhs = I::<10>::from_raw(i64::MIN);
    assert_eq!(lhs.rounded_div(-1i8), Err(E::Overflow));
    assert_eq!(lhs.raw(), i64::MIN);
}

pub fn wide_div_rounded() {
    macro_rules! check {
        ($lhs:expr,$rhs:expr,$expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            assert_eq!(lhs.rounded_div(rhs).unwrap().raw(), $expected);
            assert_eq!(lhs.raw(), $expected);
            assert_eq!(rhs.raw() as u64, u64::from(u32::MAX));
        }};
    }
    check!(
        U::<100>::from_raw(u64::MAX),
        I::<{ u32::MAX }>::from_raw(i64::from(u32::MAX)),
        u64::MAX
    );
    check!(
        I::<100>::from_raw(i64::MIN),
        U::<{ u32::MAX }>::from_raw(u64::from(u32::MAX)),
        i64::MIN
    );
}

pub fn range_div_saturating() {
    macro_rules! check {
        ($lhs:expr,$half:expr,$bound:expr) => {{
            let lhs = $lhs;
            let half = $half;
            let before = lhs.raw();
            assert_eq!(lhs.saturating_div(half).unwrap().raw(), $bound);
            assert_eq!(lhs.saturating_div_with(half).unwrap().raw(), $bound);
            assert_eq!(lhs.raw(), before);
            assert_eq!(half.raw(), 1);
        }};
    }
    check!(U::<10>::from_raw(u64::MAX), I::<2>::from_raw(1), u64::MAX);
    check!(I::<10>::from_raw(i64::MAX), U::<2>::from_raw(1), i64::MAX);
    check!(I::<10>::from_raw(i64::MIN), U::<2>::from_raw(1), i64::MIN);
    let lhs = I::<10>::from_raw(i64::MIN);
    assert_eq!(lhs.saturating_div(-1i8).unwrap().raw(), i64::MAX);
    assert_eq!(lhs.raw(), i64::MIN);
}

pub fn wide_div_saturating() {
    macro_rules! check {
        ($lhs:expr,$rhs:expr,$expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            assert_eq!(lhs.saturating_div(rhs).unwrap().raw(), $expected);
            assert_eq!(lhs.raw(), $expected);
            assert_eq!(rhs.raw() as u64, u64::from(u32::MAX));
        }};
    }
    check!(
        U::<100>::from_raw(u64::MAX),
        I::<{ u32::MAX }>::from_raw(i64::from(u32::MAX)),
        u64::MAX
    );
    check!(
        I::<100>::from_raw(i64::MIN),
        U::<{ u32::MAX }>::from_raw(u64::from(u32::MAX)),
        i64::MIN
    );
}

pub fn negative_primitive_div() {
    let lhs = I::<100>::from_raw(150);
    let result: I<100> = lhs.checked_div(-2i8).unwrap();
    assert_eq!(result.raw(), -75);
    assert_eq!(lhs.raw(), 150);
}

pub fn rounding_crosses_bound_div() {
    // 精确乘积：1190112520884487201*31=2*MAX_U+1；6148914691236517205*3=2*MAX_I+1；67280421310721*274177=2*abs(MIN_I)+1。
    macro_rules! check {
        ($lhs:expr,$rhs:expr,$expected:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            let before = (lhs.raw(), rhs.raw());
            assert_eq!(lhs.checked_div(rhs).unwrap().raw(), $expected);
            assert_eq!(lhs.checked_div_with(rhs).unwrap().raw(), $expected);
            assert_eq!(lhs.rounded_div(rhs), Err(E::Overflow));
            assert_eq!(lhs.rounded_div_with(rhs), Err(E::Overflow));
            assert_eq!((lhs.raw(), rhs.raw()), before);
        }};
    }
    check!(
        U::<10>::from_raw(1190112520884487201),
        I::<31>::from_raw(2),
        u64::MAX
    );
    check!(
        I::<10>::from_raw(6148914691236517205),
        U::<3>::from_raw(2),
        i64::MAX
    );
    check!(
        I::<10>::from_raw(-67280421310721),
        U::<274177>::from_raw(2),
        i64::MIN
    );
}

pub fn zero_divisor_checked() {
    macro_rules! check {
        ($lhs:expr,$rhs:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            assert_eq!(lhs.checked_div(rhs), Err(E::DivisionByZero));
            assert_eq!(lhs.checked_div_with(rhs), Err(E::DivisionByZero));
            assert_eq!(lhs.raw(), 150);
        }};
    }
    let i = I::<10>::from_raw(0);
    let u = U::<10>::from_raw(0);
    check!(U::<100>::from_raw(150), i);
    check!(I::<100>::from_raw(150), u);
    check!(U::<100>::from_raw(150), 0i8);
    check!(I::<100>::from_raw(150), 0u64);
    assert_eq!((i.raw(), u.raw()), (0, 0));
}

pub fn zero_divisor_rounded() {
    macro_rules! check {
        ($lhs:expr,$rhs:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            assert_eq!(lhs.rounded_div(rhs), Err(E::DivisionByZero));
            assert_eq!(lhs.rounded_div_with(rhs), Err(E::DivisionByZero));
            assert_eq!(lhs.raw(), 150);
        }};
    }
    let i = I::<10>::from_raw(0);
    let u = U::<10>::from_raw(0);
    check!(U::<100>::from_raw(150), i);
    check!(I::<100>::from_raw(150), u);
    check!(U::<100>::from_raw(150), 0i8);
    check!(I::<100>::from_raw(150), 0u64);
    assert_eq!((i.raw(), u.raw()), (0, 0));
}

pub fn zero_divisor_saturating() {
    macro_rules! check {
        ($lhs:expr,$rhs:expr) => {{
            let lhs = $lhs;
            let rhs = $rhs;
            assert_eq!(lhs.saturating_div(rhs), Err(E::DivisionByZero));
            assert_eq!(lhs.saturating_div_with(rhs), Err(E::DivisionByZero));
            assert_eq!(lhs.raw(), 150);
        }};
    }
    let i = I::<10>::from_raw(0);
    let u = U::<10>::from_raw(0);
    check!(U::<100>::from_raw(150), i);
    check!(I::<100>::from_raw(150), u);
    check!(U::<100>::from_raw(150), 0i8);
    check!(I::<100>::from_raw(150), 0u64);
    assert_eq!((i.raw(), u.raw()), (0, 0));
}

#[cfg(feature = "serde")]
pub fn serde_raw_round_trip() {
    for raw in [0, 1, u64::MAX] {
        let source = U::<{ u32::MAX }>::from_raw(raw);
        let bytes = serde_json::to_vec(&source).unwrap();
        let decoded: U<{ u32::MAX }> = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(decoded.raw(), raw);
        assert_eq!(source.raw(), raw);
    }
    for raw in [i64::MIN, -1, 0, i64::MAX] {
        let source = I::<{ u32::MAX }>::from_raw(raw);
        let bytes = serde_json::to_vec(&source).unwrap();
        let decoded: I<{ u32::MAX }> = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(decoded.raw(), raw);
        assert_eq!(source.raw(), raw);
    }
}

#[cfg(feature = "fory")]
pub fn fory_raw_round_trip() {
    let mut codec = fory::Fory::builder().xlang(false).build();
    codec.register::<U<{ u32::MAX }>>(100).unwrap();
    codec.register::<I<{ u32::MAX }>>(101).unwrap();
    for raw in [0, 1, u64::MAX] {
        let source = U::<{ u32::MAX }>::from_raw(raw);
        let bytes = codec.serialize(&source).unwrap();
        let decoded: U<{ u32::MAX }> = codec.deserialize(&bytes).unwrap();
        assert_eq!(decoded.raw(), raw);
        assert_eq!(source.raw(), raw);
    }
    for raw in [i64::MIN, -1, 0, i64::MAX] {
        let source = I::<{ u32::MAX }>::from_raw(raw);
        let bytes = codec.serialize(&source).unwrap();
        let decoded: I<{ u32::MAX }> = codec.deserialize(&bytes).unwrap();
        assert_eq!(decoded.raw(), raw);
        assert_eq!(source.raw(), raw);
    }
}
