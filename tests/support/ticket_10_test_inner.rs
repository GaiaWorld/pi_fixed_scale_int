// @api:src/lib.rs
// @impl:src/math_inner.rs
// @test:tests/ticket_10_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};
type S = FixedI64<1_000_000>;
type U = FixedU64<1_000_000>;

pub fn strict_transcendentals() {
    let signed: [(i64, fn(S) -> Result<S, MathError>, i64); 5] = [
        (1_000_000, S::exp_strict, 2_718_282),
        (500_000, S::exp2_strict, 1_414_214),
        (2_000_000, S::ln_strict, 693_147),
        (3_000_000, S::log2_strict, 1_584_963),
        (2_000_000, S::log10_strict, 301_030),
    ];
    for (raw, operation, expected) in signed {
        let input = S::from_raw(raw);
        let result: S = operation(input).unwrap();
        assert_eq!(result.raw(), expected);
        assert_eq!(input.raw(), raw);
    }
    let unsigned: [(u64, fn(U) -> Result<U, MathError>, u64); 5] = [
        (1_000_000, U::exp_strict, 2_718_282),
        (500_000, U::exp2_strict, 1_414_214),
        (2_000_000, U::ln_strict, 693_147),
        (3_000_000, U::log2_strict, 1_584_963),
        (2_000_000, U::log10_strict, 301_030),
    ];
    for (raw, operation, expected) in unsigned {
        let input = U::from_raw(raw);
        let result: U = operation(input).unwrap();
        assert_eq!(result.raw(), expected);
        assert_eq!(input.raw(), raw);
    }
}

// Independent exact rational oracle: (raw/D)^n * D. No math backend or floats.
fn rational_raw(raw: i64, d: u32, n: i32) -> i128 {
    let k = n.unsigned_abs();
    let magnitude = i128::from(raw).abs().pow(k);
    let scale = i128::from(d).pow(k);
    let (numerator, denominator) = if n < 0 {
        (scale * i128::from(d), magnitude)
    } else {
        (magnitude * i128::from(d), scale)
    };
    let rounded = numerator / denominator
        + i128::from(2 * (numerator % denominator) >= denominator);
    if raw < 0 && n % 2 != 0 { -rounded } else { rounded }
}

pub fn strict_rational_power_oracle() {
    for raw in -25..=25 {
        for n in -5..=5 {
            if raw == 0 && n < 0 { continue; }
            let expected = rational_raw(raw, 10, n);
            let input = FixedI64::<10>::from_raw(raw);
            match input.powi_strict(n) {
                Ok(result) => assert_eq!(i128::from(result.raw()), expected, "raw={raw}, n={n}"),
                Err(MathError::PrecisionExhausted) => {},
                Err(error) => panic!("raw={raw}, n={n}, unexpected error={error:?}"),
            }
            assert_eq!(input.raw(), raw);
            if raw >= 0 {
                let input = FixedU64::<10>::from_raw(raw as u64);
                match input.powi_strict(n) {
                    Ok(result) => assert_eq!(i128::from(result.raw()), expected, "unsigned raw={raw}, n={n}"),
                    Err(MathError::PrecisionExhausted) => {},
                    Err(error) => panic!("unsigned raw={raw}, n={n}, unexpected error={error:?}"),
                }
                assert_eq!(input.raw(), raw as u64);
            }
        }
    }
}

pub fn strict_logarithm_quadrants() {
    for (raw, base, expected) in [
        (4_000_000, 2_000_000, 2_000_000),
        (4_000_000, 500_000, -2_000_000),
        (250_000, 2_000_000, -2_000_000),
        (250_000, 500_000, 2_000_000),
    ] {
        let x = S::from_raw(raw);
        let b = S::from_raw(base);
        let result: S = x.log_strict(b).unwrap();
        assert_eq!(result.raw(), expected);
        assert_eq!((x.raw(), b.raw()), (raw, base));
    }
}

pub fn strict_near_one_bases() {
    const D: u32 = u32::MAX;
    for raw in [i64::from(D) - 1, i64::from(D) + 1] {
        let b = FixedI64::<D>::from_raw(raw);
        assert_eq!(b.log_strict(b).unwrap().raw(), i64::from(D));
        assert_eq!(FixedI64::<D>::from_raw(i64::from(D)).log_strict(b).unwrap().raw(), 0);
        assert_eq!(b.raw(), raw);
        let b = FixedU64::<D>::from_raw(raw as u64);
        assert_eq!(b.log_strict(b).unwrap().raw(), u64::from(D));
        assert_eq!(FixedU64::<D>::from_raw(u64::from(D)).log_strict(b).unwrap().raw(), 0);
        assert_eq!(b.raw(), raw as u64);
    }
}

pub fn strict_maximum_denominator() {
    const D: u32 = u32::MAX;
    let s = FixedI64::<D>::from_raw(i64::from(D));
    let u = FixedU64::<D>::from_raw(u64::from(D));
    let sr: FixedI64<D> = s.exp_strict().unwrap();
    let ur: FixedU64<D> = u.exp_strict().unwrap();
    assert_eq!(sr.raw(), 11_674_931_552);
    assert_eq!(ur.raw(), 11_674_931_552);
    assert_eq!(s.raw(), i64::from(D));
    assert_eq!(u.raw(), u64::from(D));
    let third = FixedI64::<3>::from_raw(1);
    assert_eq!(third.exp_strict().unwrap().raw(), 4);
    assert_eq!(third.raw(), 1);
}

fn boundaries<const D: u32>() {
    for raw in [i64::MIN, i64::MAX] {
        let input = FixedI64::<D>::from_raw(raw);
        let result: FixedI64<D> = input.powi_strict(1).unwrap();
        assert_eq!(result.raw(), raw);
        assert_eq!(input.raw(), raw);
    }
    let input = FixedU64::<D>::from_raw(u64::MAX);
    let result: FixedU64<D> = input.powi_strict(1).unwrap();
    assert_eq!(result.raw(), u64::MAX);
    assert_eq!(input.raw(), u64::MAX);
}
pub fn strict_storage_boundaries() {
    boundaries::<1>();
    boundaries::<{ u32::MAX }>();
}

pub fn strict_zero_and_minimum_exponents() {
    for raw in [0, 1, 20] {
        let s = FixedI64::<10>::from_raw(raw);
        let u = FixedU64::<10>::from_raw(raw as u64);
        assert_eq!(s.powi_strict(0).unwrap().raw(), 10);
        assert_eq!(u.powi_strict(0).unwrap().raw(), 10);
        assert_eq!(s.raw(), raw);
        assert_eq!(u.raw(), raw as u64);
    }
    for raw in [-10, 10] {
        let s = FixedI64::<10>::from_raw(raw);
        assert_eq!(s.powi_strict(i32::MIN).unwrap().raw(), 10);
        assert_eq!(s.raw(), raw);
    }
    let u = FixedU64::<10>::from_raw(10);
    assert_eq!(u.powi_strict(i32::MIN).unwrap().raw(), 10);
    assert_eq!(u.raw(), 10);
}

pub fn strict_invalid_logarithms() {
    for raw in [-1, 0] {
        let s = FixedI64::<10>::from_raw(raw);
        for result in [s.ln_strict(), s.log2_strict(), s.log10_strict(), s.log_strict(FixedI64::from_raw(20))] {
            assert_eq!(result, Err(MathError::Domain));
        }
        assert_eq!(s.raw(), raw);
    }
    let u = FixedU64::<10>::from_raw(0);
    for result in [u.ln_strict(), u.log2_strict(), u.log10_strict(), u.log_strict(FixedU64::from_raw(20))] {
        assert_eq!(result, Err(MathError::Domain));
    }
    assert_eq!(u.raw(), 0);
}

pub fn strict_invalid_bases_and_zero_negative_power() {
    let s = FixedI64::<10>::from_raw(20);
    for raw in [-10, 0, 10] {
        let b = FixedI64::from_raw(raw);
        assert_eq!(s.log_strict(b), Err(MathError::Domain));
        assert_eq!(b.raw(), raw);
    }
    let u = FixedU64::<10>::from_raw(20);
    for raw in [0, 10] {
        let b = FixedU64::from_raw(raw);
        assert_eq!(u.log_strict(b), Err(MathError::Domain));
        assert_eq!(b.raw(), raw);
    }
    assert_eq!((s.raw(), u.raw()), (20, 20));
    for n in [-1, i32::MIN] {
        let s = FixedI64::<10>::from_raw(0);
        let u = FixedU64::<10>::from_raw(0);
        assert_eq!(s.powi_strict(n), Err(MathError::Domain));
        assert_eq!(u.powi_strict(n), Err(MathError::Domain));
        assert_eq!((s.raw(), u.raw()), (0, 0));
    }
}

pub fn strict_unsigned_rounded_range() {
    let x = FixedU64::<2>::from_raw(1);
    let b = FixedU64::<2>::from_raw(200);
    assert_eq!(x.log_strict(b).unwrap().raw(), 0);
    assert_eq!((x.raw(), b.raw()), (1, 200));
    let x = FixedU64::<100>::from_raw(50);
    for result in [x.ln_strict(), x.log2_strict(), x.log10_strict()] {
        assert_eq!(result, Err(MathError::Overflow));
    }
    assert_eq!(x.raw(), 50);
}

pub fn strict_exponential_extremes() {
    for raw in [-101, -100, i64::MIN] {
        let x = FixedI64::<1>::from_raw(raw);
        assert_eq!(x.exp_strict().unwrap().raw(), 0);
        assert_eq!(x.exp2_strict().unwrap().raw(), 0);
        assert_eq!(x.raw(), raw);
    }
    for raw in [100, 101, i64::MAX] {
        let x = FixedI64::<1>::from_raw(raw);
        assert_eq!(x.exp_strict(), Err(MathError::Overflow));
        assert_eq!(x.exp2_strict(), Err(MathError::Overflow));
        assert_eq!(x.raw(), raw);
    }
    for raw in [100, 101, u64::MAX] {
        let x = FixedU64::<1>::from_raw(raw);
        assert_eq!(x.exp_strict(), Err(MathError::Overflow));
        assert_eq!(x.exp2_strict(), Err(MathError::Overflow));
        assert_eq!(x.raw(), raw);
    }
}

pub fn strict_binary_rounding_and_overflow() {
    for (raw, expected) in [(-2, 0), (-1, 1), (62, 4_611_686_018_427_387_904)] {
        let x = FixedI64::<1>::from_raw(raw);
        assert_eq!(x.exp2_strict().unwrap().raw(), expected);
        assert_eq!(x.raw(), raw);
    }
    let x = FixedI64::<1>::from_raw(63);
    assert_eq!(x.exp2_strict(), Err(MathError::Overflow));
    assert_eq!(x.raw(), 63);
    let x = FixedU64::<1>::from_raw(63);
    assert_eq!(x.exp2_strict().unwrap().raw(), 9_223_372_036_854_775_808);
    assert_eq!(x.raw(), 63);
    let x = FixedU64::<1>::from_raw(64);
    assert_eq!(x.exp2_strict(), Err(MathError::Overflow));
    assert_eq!(x.raw(), 64);
    let x = FixedU64::<1>::from_raw(u64::MAX);
    assert_eq!(x.log2_strict().unwrap().raw(), 64);
    assert_eq!(x.raw(), u64::MAX);
}
