use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};

type I = FixedI64<1_000_000>;
type U = FixedU64<1_000_000>;

fn near(actual: i128, expected: i128) {
    assert!(
        (actual - expected).abs() <= 1,
        "actual={actual}, correct rounded raw={expected}"
    );
}

pub fn ordinary_transcendentals() {
    // Independent decimal constants, not outputs from the strict/backend implementation.
    for (raw, op, expected) in [
        (1_000_000, 0, 2_718_282),
        (500_000, 1, 1_414_214),
        (2_000_000, 2, 693_147),
        (3_000_000, 3, 1_584_963),
        (2_000_000, 4, 301_030),
    ] {
        let x = I::from_raw(raw);
        let result: I = match op {
            0 => x.exp(),
            1 => x.exp2(),
            2 => x.ln(),
            3 => x.log2(),
            _ => x.log10(),
        }
        .unwrap();
        near(result.raw().into(), expected);
        assert_eq!(x.raw(), raw);
        let x = U::from_raw(raw as u64);
        let result: U = match op {
            0 => x.exp(),
            1 => x.exp2(),
            2 => x.ln(),
            3 => x.log2(),
            _ => x.log10(),
        }
        .unwrap();
        near(result.raw().into(), expected);
        assert_eq!(x.raw(), raw as u64);
    }
}

pub fn logarithm_sign_quadrants() {
    for (raw, base, expected) in [
        (4_000_000, 2_000_000, 2_000_000),
        (4_000_000, 500_000, -2_000_000),
        (250_000, 2_000_000, -2_000_000),
        (250_000, 500_000, 2_000_000),
    ] {
        let x = I::from_raw(raw);
        let b = I::from_raw(base);
        let result: I = x.log(b).unwrap();
        near(result.raw().into(), expected);
        assert_eq!((x.raw(), b.raw()), (raw, base));
    }
}

fn rounded_rational_power(raw: i64, d: u32, n: i32) -> i128 {
    // Exact rational: (raw / D)^n * D. No floating-point or library math oracle.
    let k = n.unsigned_abs();
    let a = i128::from(raw).abs().pow(k);
    let b = i128::from(d).pow(k);
    let (num, den) = if n < 0 {
        (b * i128::from(d), a)
    } else {
        (a * i128::from(d), b)
    };
    let magnitude = num / den + i128::from((num % den) * 2 >= den);
    if raw < 0 && n % 2 != 0 {
        -magnitude
    } else {
        magnitude
    }
}

pub fn rational_power_oracle() {
    for raw in [-21, -20, -19, -5, 0, 5, 19, 20, 21] {
        for n in [-3, -2, -1, 1, 2, 3] {
            if raw == 0 && n < 0 {
                continue;
            }
            let expected = rounded_rational_power(raw, 10, n);
            let x = FixedI64::<10>::from_raw(raw);
            let result: FixedI64<10> = x.powi(n).unwrap();
            near(result.raw().into(), expected);
            assert_eq!(x.raw(), raw);
            if raw >= 0 {
                let x = FixedU64::<10>::from_raw(raw as u64);
                let result: FixedU64<10> = x.powi(n).unwrap();
                near(result.raw().into(), expected);
                assert_eq!(x.raw(), raw as u64);
            }
        }
    }
}

pub fn zero_power_is_one() {
    for raw in [0, 1, 20] {
        let x = FixedI64::<10>::from_raw(raw);
        near(x.powi(0).unwrap().raw().into(), 10);
        assert_eq!(x.raw(), raw);
        let x = FixedU64::<10>::from_raw(raw as u64);
        near(x.powi(0).unwrap().raw().into(), 10);
        assert_eq!(x.raw(), raw as u64);
    }
    let x = FixedI64::<{ u32::MAX }>::from_raw(0);
    near(x.powi(0).unwrap().raw().into(), u32::MAX.into());
    assert_eq!(x.raw(), 0);
    let x = FixedU64::<{ u32::MAX }>::from_raw(0);
    near(x.powi(0).unwrap().raw().into(), u32::MAX.into());
    assert_eq!(x.raw(), 0);
}

pub fn invalid_logarithms() {
    for raw in [0, -1] {
        let x = FixedI64::<10>::from_raw(raw);
        for result in [x.ln(), x.log2(), x.log10(), x.log(FixedI64::from_raw(20))] {
            assert_eq!(result, Err(MathError::Domain));
        }
        assert_eq!(x.raw(), raw);
    }
    let x = FixedU64::<10>::from_raw(0);
    for result in [x.ln(), x.log2(), x.log10(), x.log(FixedU64::from_raw(20))] {
        assert_eq!(result, Err(MathError::Domain));
    }
    assert_eq!(x.raw(), 0);
}

pub fn invalid_bases() {
    let x = FixedI64::<10>::from_raw(20);
    for raw in [-10, 0, 10] {
        let b = FixedI64::from_raw(raw);
        assert_eq!(x.log(b), Err(MathError::Domain));
        assert_eq!((x.raw(), b.raw()), (20, raw));
    }
    let x = FixedU64::<10>::from_raw(20);
    for raw in [0, 10] {
        let b = FixedU64::from_raw(raw);
        assert_eq!(x.log(b), Err(MathError::Domain));
        assert_eq!((x.raw(), b.raw()), (20, raw));
    }
}

pub fn zero_negative_power() {
    for n in [-1, i32::MIN] {
        let x = FixedI64::<10>::from_raw(0);
        assert_eq!(x.powi(n), Err(MathError::Domain));
        assert_eq!(x.raw(), 0);
        let x = FixedU64::<10>::from_raw(0);
        assert_eq!(x.powi(n), Err(MathError::Domain));
        assert_eq!(x.raw(), 0);
    }
}

pub fn unsigned_rounded_range() {
    let x = FixedU64::<2>::from_raw(1);
    let b = FixedU64::<2>::from_raw(200);
    near(x.log(b).unwrap().raw().into(), 0);
    assert_eq!((x.raw(), b.raw()), (1, 200));
    let x = FixedU64::<100>::from_raw(50);
    for result in [x.ln(), x.log2(), x.log10()] {
        assert_eq!(result, Err(MathError::Overflow));
    }
    assert_eq!(x.raw(), 50);
}

pub fn exponential_extremes() {
    for raw in [-101, -100, 100, 101] {
        let x = FixedI64::<1>::from_raw(raw);
        let operations: [(&str, fn(FixedI64<1>) -> Result<FixedI64<1>, MathError>); 2] =
            [("exp", FixedI64::<1>::exp), ("exp2", FixedI64::<1>::exp2)];
        for (name, operation) in operations {
            eprintln!("exponential_extremes: signed raw={raw}, method={name}");
            let result: Result<FixedI64<1>, MathError> = operation(x);
            if raw < 0 {
                near(result.unwrap().raw().into(), 0);
            } else {
                assert_eq!(result, Err(MathError::Overflow));
            }
        }
        assert_eq!(x.raw(), raw);
    }
    for raw in [100, 101] {
        let x = FixedU64::<1>::from_raw(raw);
        for result in [x.exp(), x.exp2()] {
            assert_eq!(result, Err(MathError::Overflow));
        }
        assert_eq!(x.raw(), raw);
    }
}

pub fn power_storage_boundaries() {
    for raw in [i64::MIN, i64::MAX] {
        let x = FixedI64::<1>::from_raw(raw);
        near(x.powi(1).unwrap().raw().into(), raw.into());
        assert_eq!(x.powi(2), Err(MathError::Overflow));
        assert_eq!(x.raw(), raw);
    }
    let x = FixedU64::<1>::from_raw(u64::MAX);
    near(x.powi(1).unwrap().raw().into(), u64::MAX.into());
    assert_eq!(x.powi(2), Err(MathError::Overflow));
    assert_eq!(x.raw(), u64::MAX);
}

pub fn minimum_exponent() {
    for raw in [-10, 10] {
        let x = FixedI64::<10>::from_raw(raw);
        near(x.powi(i32::MIN).unwrap().raw().into(), 10);
        assert_eq!(x.raw(), raw);
    }
    let x = FixedU64::<10>::from_raw(10);
    near(x.powi(i32::MIN).unwrap().raw().into(), 10);
    assert_eq!(x.raw(), 10);
}

pub fn maximum_denominator() {
    let x = FixedI64::<{ u32::MAX }>::from_raw(i64::from(u32::MAX));
    let result: FixedI64<{ u32::MAX }> = x.exp().unwrap();
    near(result.raw().into(), 11_674_931_552);
    assert_eq!(x.raw(), i64::from(u32::MAX));
    let x = FixedU64::<{ u32::MAX }>::from_raw(u64::from(u32::MAX));
    let result: FixedU64<{ u32::MAX }> = x.exp().unwrap();
    near(result.raw().into(), 11_674_931_552);
    assert_eq!(x.raw(), u64::from(u32::MAX));
    let x = FixedI64::<3>::from_raw(1);
    near(x.exp().unwrap().raw().into(), 4);
    assert_eq!(x.raw(), 1);
}

pub fn near_one_log_base() {
    // For D=2^32-1, D*ln(1+1/D)/ln(1-1/D) lies within
    // (-D+1-1/D, -D+1+1/D), hence correct rounded raw is -D+1.
    let x = FixedI64::<{ u32::MAX }>::from_raw(4_294_967_296);
    for (base, expected) in [
        (4_294_967_296, 4_294_967_295),
        (4_294_967_294, -4_294_967_294),
    ] {
        let b = FixedI64::from_raw(base);
        near(x.log(b).unwrap().raw().into(), expected);
        assert_eq!((x.raw(), b.raw()), (4_294_967_296, base));
    }
    let x = FixedU64::<{ u32::MAX }>::from_raw(4_294_967_296);
    let b = FixedU64::from_raw(4_294_967_296);
    near(x.log(b).unwrap().raw().into(), 4_294_967_295);
    assert_eq!((x.raw(), b.raw()), (4_294_967_296, 4_294_967_296));
    let b = FixedU64::from_raw(4_294_967_294);
    assert_eq!(x.log(b), Err(MathError::Overflow));
    assert_eq!((x.raw(), b.raw()), (4_294_967_296, 4_294_967_294));
}
