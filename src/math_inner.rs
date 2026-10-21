use crate::{FixedI64, FixedU64};
use astro_float::{BigFloat, Consts, RoundingMode as R};

/// Failures from the optional high-precision mathematical operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MathError {
    Domain,
    Overflow,
    PrecisionExhausted,
    ComputationFailed,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for MathError {}

#[derive(Clone, Copy)]
enum Function {
    Exp,
    Exp2,
    Ln,
    Log2,
    Log10,
    Log(i128),
    Pow(i32),
}

struct Interval {
    lo: BigFloat,
    hi: BigFloat,
}

fn integer(n: i128, p: usize) -> BigFloat {
    BigFloat::from_i128(n, p)
}

fn ratio(n: i128, d: u32, p: usize) -> Interval {
    let n = integer(n, p);
    let d = BigFloat::from_u32(d, p);
    Interval {
        lo: n.div(&d, p, R::Down),
        hi: n.div(&d, p, R::Up),
    }
}

fn divide(a: Interval, b: Interval, p: usize) -> Result<Interval, MathError> {
    let zero = integer(0, p);
    if b.lo.cmp(&zero).is_some_and(|n| n <= 0) && b.hi.cmp(&zero).is_some_and(|n| n >= 0) {
        return Err(MathError::PrecisionExhausted);
    }
    let lows = [
        a.lo.div(&b.lo, p, R::Down),
        a.lo.div(&b.hi, p, R::Down),
        a.hi.div(&b.lo, p, R::Down),
        a.hi.div(&b.hi, p, R::Down),
    ];
    let highs = [
        a.lo.div(&b.lo, p, R::Up),
        a.lo.div(&b.hi, p, R::Up),
        a.hi.div(&b.lo, p, R::Up),
        a.hi.div(&b.hi, p, R::Up),
    ];
    let lo = lows
        .into_iter()
        .reduce(|a, b| a.min(&b))
        .ok_or(MathError::ComputationFailed)?;
    let hi = highs
        .into_iter()
        .reduce(|a, b| a.max(&b))
        .ok_or(MathError::ComputationFailed)?;
    Ok(Interval { lo, hi })
}

fn evaluate(
    raw: i128,
    d: u32,
    function: Function,
    p: usize,
    cc: &mut Consts,
) -> Result<Interval, MathError> {
    let x = ratio(raw, d, p);
    let result = match function {
        Function::Exp => Interval {
            lo: x.lo.exp(p, R::Down, cc),
            hi: x.hi.exp(p, R::Up, cc),
        },
        Function::Exp2 => {
            let two = integer(2, p);
            Interval {
                lo: two.pow(&x.lo, p, R::Down, cc),
                hi: two.pow(&x.hi, p, R::Up, cc),
            }
        }
        Function::Ln => Interval {
            lo: x.lo.ln(p, R::Down, cc),
            hi: x.hi.ln(p, R::Up, cc),
        },
        Function::Log2 => Interval {
            lo: x.lo.log2(p, R::Down, cc),
            hi: x.hi.log2(p, R::Up, cc),
        },
        Function::Log10 => Interval {
            lo: x.lo.log10(p, R::Down, cc),
            hi: x.hi.log10(p, R::Up, cc),
        },
        Function::Log(base) => {
            let b = ratio(base, d, p);
            divide(
                Interval {
                    lo: x.lo.ln(p, R::Down, cc),
                    hi: x.hi.ln(p, R::Up, cc),
                },
                Interval {
                    lo: b.lo.ln(p, R::Down, cc),
                    hi: b.hi.ln(p, R::Up, cc),
                },
                p,
            )?
        }
        Function::Pow(n) => {
            if n == 0 {
                return Ok(Interval {
                    lo: integer(d as i128, p),
                    hi: integer(d as i128, p),
                });
            }
            // Work on |x| so integer powers are monotone. Restore sign only at the end.
            let a = ratio(raw.abs(), d, p);
            let mut power = Interval {
                lo: a.lo.powi(n.unsigned_abs() as usize, p, R::Down),
                hi: a.hi.powi(n.unsigned_abs() as usize, p, R::Up),
            };
            if n < 0 {
                power = Interval {
                    lo: power.hi.reciprocal(p, R::Down),
                    hi: power.lo.reciprocal(p, R::Up),
                };
            }
            if raw < 0 && n % 2 != 0 {
                power = Interval {
                    lo: power.hi.neg(),
                    hi: power.lo.neg(),
                };
            }
            power
        }
    };
    let scale = BigFloat::from_u32(d, p);
    Ok(Interval {
        lo: result.lo.mul(&scale, p, R::Down),
        hi: result.hi.mul(&scale, p, R::Up),
    })
}

// Quantize the binary value directly, without decimal formatting or f64 conversion.
// Sentinels are beyond every supported 64-bit raw count.
fn quantize(value: &BigFloat, p: usize) -> Result<i128, MathError> {
    if value.is_nan() {
        return Err(MathError::ComputationFailed);
    }
    const LIMIT: i128 = 1i128 << 65;
    if value.is_inf_pos() || value.cmp(&integer(LIMIT, p)).is_some_and(|n| n >= 0) {
        return Ok(LIMIT);
    }
    if value.is_inf_neg() || value.cmp(&integer(-LIMIT, p)).is_some_and(|n| n <= 0) {
        return Ok(-LIMIT);
    }
    let negative = value.is_negative();
    let magnitude = value.abs();
    let (mut low, mut high) = (0i128, LIMIT);
    while low + 1 < high {
        let middle = (low + high) / 2;
        match magnitude.cmp(&integer(middle, p)) {
            Some(n) if n >= 0 => low = middle,
            Some(_) => high = middle,
            None => return Err(MathError::ComputationFailed),
        }
    }
    let half = integer(2 * low + 1, p).div(&integer(2, p), p, R::ToEven);
    if magnitude.cmp(&half).ok_or(MathError::ComputationFailed)? >= 0 {
        low += 1;
    }
    Ok(if negative { -low } else { low })
}

fn compute(
    raw: i128,
    d: u32,
    function: Function,
    strict: bool,
    min: i128,
    max: i128,
) -> Result<i128, MathError> {
    match function {
        Function::Ln | Function::Log2 | Function::Log10 if raw <= 0 => {
            return Err(MathError::Domain)
        }
        Function::Log(base) if raw <= 0 || base <= 0 || base == i128::from(d) => {
            return Err(MathError::Domain)
        }
        Function::Pow(n) if raw == 0 && n < 0 => return Err(MathError::Domain),
        // At |x|>100 both exponential bases are safely outside all 64-bit raw ranges.
        Function::Exp | Function::Exp2 if raw > 100 * i128::from(d) => {
            return Err(MathError::Overflow)
        }
        Function::Exp | Function::Exp2 if raw < -100 * i128::from(d) => return Ok(0),
        _ => {}
    }
    let mut cc = Consts::new().map_err(|_| MathError::ComputationFailed)?;
    let mut p = if strict { 192 } else { 128 };
    while p <= 12288 {
        let bounds = match evaluate(raw, d, function, p, &mut cc) {
            Err(MathError::PrecisionExhausted) => {
                p *= 2;
                continue;
            }
            other => other?,
        };
        let lo = quantize(&bounds.lo, p)?;
        let hi = quantize(&bounds.hi, p)?;
        if lo > max || hi < min {
            return Err(MathError::Overflow);
        }
        if lo == hi || (!strict && hi - lo <= 1 && lo >= min && hi <= max) {
            return if lo >= min && lo <= max {
                Ok(lo)
            } else {
                Err(MathError::Overflow)
            };
        }
        p *= 2;
    }
    Err(MathError::PrecisionExhausted)
}

macro_rules! unary {
    ($name:ident,$strict:ident,$kind:ident) => {
        /// High-precision result within one raw unit of correct half-away rounding.
        /// Keeps this type and denominator. Requires the optional math feature.
        pub fn $name(self) -> Result<Self, MathError> {
            self.math(Function::$kind, false)
        }
        /// Correctly rounded result (half ties away from zero), certified by bounds.
        /// Returns PrecisionExhausted rather than an uncertified result.
        pub fn $strict(self) -> Result<Self, MathError> {
            self.math(Function::$kind, true)
        }
    };
}

macro_rules! math_methods {
    ($name:ident,$ty:ty) => {
        impl<const D: u32> $name<D> {
            fn math(self, function: Function, strict: bool) -> Result<Self, MathError> {
                const { assert!(D > 0, "denominator must be positive") };
                compute(
                    self.raw() as i128,
                    D,
                    function,
                    strict,
                    <$ty>::MIN as i128,
                    <$ty>::MAX as i128,
                )
                .map(|raw| Self::from_raw(raw as $ty))
            }
            unary!(exp, exp_strict, Exp);
            unary!(exp2, exp2_strict, Exp2);
            unary!(ln, ln_strict, Ln);
            unary!(log2, log2_strict, Log2);
            unary!(log10, log10_strict, Log10);
            /// Logarithm in a positive base other than one. At most one raw unit error.
            pub fn log(self, base: Self) -> Result<Self, MathError> {
                self.math(Function::Log(base.raw() as i128), false)
            }
            /// Correctly rounded logarithm in a positive base other than one.
            pub fn log_strict(self, base: Self) -> Result<Self, MathError> {
                self.math(Function::Log(base.raw() as i128), true)
            }
            /// Integer power; negative exponents are reciprocals. Defines 0^0 as 1.
            /// Error is at most one raw unit relative to correct half-away rounding.
            pub fn powi(self, n: i32) -> Result<Self, MathError> {
                self.math(Function::Pow(n), false)
            }
            /// Correctly rounded integer power; defines 0^0 as 1.
            pub fn powi_strict(self, n: i32) -> Result<Self, MathError> {
                self.math(Function::Pow(n), true)
            }
        }
    };
}
math_methods!(FixedI64, i64);
math_methods!(FixedU64, u64);
