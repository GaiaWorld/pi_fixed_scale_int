// 设计：pi-fixed-scale-int
// Frozen public declarations. Arithmetic implementations belong in fixed_inner.rs.

mod fixed_inner;
use fixed_inner::{Bound, Op, Rounding};
use std::fmt;
use std::str::FromStr;

/// A fixed-point value or integer usable as the right operand of multiplication or division.
/// The returned pair is the signed numerator and positive denominator.
pub trait FixedOperand {
    fn parts(self) -> (i128, u32);
}

macro_rules! integer_operands {
    ($($ty:ty),* $(,)?) => {$ (
        impl FixedOperand for $ty {
            fn parts(self) -> (i128, u32) { (self as i128, 1) }
        }
    )*};
}
integer_operands!(u8, u16, u32, u64, i8, i16, i32, i64);

/// Invalid decimal text or a value outside the raw count's range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseFixedError {
    Invalid,
    Overflow,
}

impl fmt::Display for ParseFixedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ParseFixedError {}

pub const SCALE_10: u32 = 10;
pub const SCALE_100: u32 = 100;
pub const SCALE_1000: u32 = 1_000;
pub const SCALE_10000: u32 = 10_000;
pub const SCALE_100000: u32 = 100_000;
pub const SCALE_1000000: u32 = 1_000_000;
pub const SCALE_10000000: u32 = 10_000_000;
pub const SCALE_100000000: u32 = 100_000_000;
pub const SCALE_75600: u32 = 75_600; // 2**4 * 3**3 * 5**2 * 7
pub const SCALE_324000: u32 = 324_000; // 2**5 * 3**4 * 5**3
pub const SCALE_1371600: u32 = 1_371_600; // 2**4 * 3**3 * 5**2 * 127
pub const SCALE_2268000: u32 = 2_268_000; // 2**5 * 3**4 * 5**3 * 7
pub const SCALE_9601200: u32 = 9_601_200; // 2**4 * 3**3 * 5**2 * 7 * 127

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticError {
    Overflow,
    DivisionByZero,
}

macro_rules! fixed_declarations {
    ($name:ident, $ty:ty, $integer:ident, $arithmetic:ident, $rescale:ident, $to_integer:ident) => {
        /// Public value is raw / D, with raw in the named 64-bit integer domain.
        /// D is a positive u32 constant when public methods or comparisons are used.
        /// This Copy value owns a private count, borrows nothing, and is immutable.
        /// Add/subtract require identical types; multiply/divide accept other
        /// signedness or denominators and keep the left operand's type and scale.
        #[derive(Clone, Copy)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "fory", derive(fory::ForyStruct))]
        pub struct $name<const D: u32> { raw: $ty }

        impl<const D: u32> fmt::Debug for $name<D> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name)).field("raw", &self.raw).field("denominator", &D).finish()
            }
        }

        impl<const D: u32> fmt::Display for $name<D> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::display_decimal(self.raw as i128, D, f)
            }
        }

        impl<const D: u32> FromStr for $name<D> {
            type Err = ParseFixedError;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                const { assert!(D > 0, "denominator must be positive") };
                let count = fixed_inner::parse_decimal(value, D)?;
                let raw = <$ty>::try_from(count).map_err(|_| ParseFixedError::Overflow)?;
                Ok(Self::from_raw(raw))
            }
        }

        impl<const D: u32> FixedOperand for $name<D> {
            fn parts(self) -> (i128, u32) {
                const { assert!(D > 0, "denominator must be positive") };
                (self.raw as i128, D)
            }
        }

        impl<const D: u32> TryFrom<f64> for $name<D> {
            type Error = ArithmeticError;
            fn try_from(value: f64) -> Result<Self, Self::Error> {
                Self::try_from_f64(value)
            }
        }

        impl<const D: u32> TryFrom<f32> for $name<D> {
            type Error = ArithmeticError;
            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::try_from_f32(value)
            }
        }

        impl<const D: u32> $name<D> {
            /// Stores a raw count (units of 1/D) without scaling. No runtime failure;
            /// using D=0 fails compilation. Does not mutate input or external state.
            pub fn from_raw(raw: $ty) -> Self {
                const { assert!(D > 0, "denominator must be positive") };
                Self { raw }
            }

            /// Reads the raw count in units of 1/D without conversion or mutation.
            /// Cannot fail at runtime; using D=0 fails compilation.
            pub fn raw(self) -> $ty {
                const { assert!(D > 0, "denominator must be positive") };
                self.raw
            }

            /// Converts a whole integer to raw count value * D; out-of-range returns
            /// Overflow, leaving input and external state unchanged.
            pub fn try_from_integer(value: $ty) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$integer::<D>(value).map(Self::from_raw)
            }

            /// Converts a finite float to the closest raw count; ties round away from zero.
            pub fn try_from_f64(value: f64) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                if !value.is_finite() { return Err(ArithmeticError::Overflow); }
                let count = fixed_inner::parse_decimal(&value.to_string(), D)
                    .map_err(|_| ArithmeticError::Overflow)?;
                <$ty>::try_from(count).map(Self::from_raw).map_err(|_| ArithmeticError::Overflow)
            }

            /// Converts any supported primitive integer to this scale without loss.
            pub fn try_from_number<I: Into<i128>>(value: I) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let count = value.into().checked_mul(i128::from(D)).ok_or(ArithmeticError::Overflow)?;
                <$ty>::try_from(count).map(Self::from_raw).map_err(|_| ArithmeticError::Overflow)
            }

            /// Converts a 32-bit float to a raw count, with half ties away from zero.
            pub fn try_from_f32(value: f32) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                if !value.is_finite() { return Err(ArithmeticError::Overflow); }
                let count = fixed_inner::parse_decimal(&value.to_string(), D)
                    .map_err(|_| ArithmeticError::Overflow)?;
                <$ty>::try_from(count).map(Self::from_raw).map_err(|_| ArithmeticError::Overflow)
            }

            /// Multiplies values of different signedness/scales or a primitive integer.
            /// The result has the left operand's type and denominator.
            pub fn checked_mul_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Mul, Rounding::Truncate, Bound::Checked, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// Divides by a value of another integer type/scale or a primitive integer.
            pub fn checked_div_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Div, Rounding::Truncate, Bound::Checked, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// Multiplies and rounds half ties away from zero.
            pub fn rounded_mul_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Mul, Rounding::HalfAway, Bound::Checked, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// Divides and rounds half ties away from zero.
            pub fn rounded_div_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Div, Rounding::HalfAway, Bound::Checked, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// Multiplies and clamps an out-of-range count to the result type's bounds.
            pub fn saturating_mul_with<R: FixedOperand>(self, rhs: R) -> Self {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                Self::from_raw(fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Mul, Rounding::Truncate, Bound::Saturate, <$ty>::MIN as i128, <$ty>::MAX as u128).expect("multiplication cannot divide by zero"))
            }

            /// Divides and clamps an out-of-range count; zero divisors return an error.
            pub fn saturating_div_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Div, Rounding::Truncate, Bound::Saturate, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// Adds raw counts in units of 1/D. Out-of-range returns Overflow;
            /// neither input nor external state changes on success or failure.
            pub fn checked_add(self, rhs: Self) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$arithmetic::<D>(self.raw, rhs.raw, Op::Add, Rounding::Truncate, Bound::Checked).map(Self::from_raw)
            }

            /// Subtracts raw counts in units of 1/D. Out-of-range returns Overflow;
            /// neither input nor external state changes on success or failure.
            pub fn checked_sub(self, rhs: Self) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$arithmetic::<D>(self.raw, rhs.raw, Op::Sub, Rounding::Truncate, Bound::Checked).map(Self::from_raw)
            }

            /// Multiplies values, truncating toward zero in the left operand's scale.
            /// An out-of-range final count returns Overflow; inputs stay unchanged.
            pub fn checked_mul<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.checked_mul_with(rhs)
            }

            /// Divides values, truncating toward zero in the left operand's scale.
            /// Zero divisor returns DivisionByZero; out-of-range count returns Overflow.
            /// Neither error changes inputs or external state.
            pub fn checked_div<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.checked_div_with(rhs)
            }

            /// Adds raw counts, clamping above MAX or below MIN (u64 MIN is zero).
            /// Never errors; neither input nor external state changes.
            pub fn saturating_add(self, rhs: Self) -> Self {
                const { assert!(D > 0, "denominator must be positive") };
                Self::from_raw(fixed_inner::$arithmetic::<D>(self.raw, rhs.raw, Op::Add, Rounding::Truncate, Bound::Saturate).expect("addition cannot divide by zero"))
            }

            /// Subtracts raw counts, clamping above MAX or below MIN (u64 MIN is zero).
            /// Never errors; neither input nor external state changes.
            pub fn saturating_sub(self, rhs: Self) -> Self {
                const { assert!(D > 0, "denominator must be positive") };
                Self::from_raw(fixed_inner::$arithmetic::<D>(self.raw, rhs.raw, Op::Sub, Rounding::Truncate, Bound::Saturate).expect("subtraction cannot divide by zero"))
            }

            /// Computes raw1 * raw2 / D, truncates toward zero, then clamps the
            /// final count to the 64-bit bounds. Never errors or mutates inputs.
            pub fn saturating_mul<R: FixedOperand>(self, rhs: R) -> Self {
                self.saturating_mul_with(rhs)
            }

            /// Computes raw1 * D / raw2, truncates toward zero and clamps the
            /// final count to 64-bit bounds. Zero divisor returns DivisionByZero;
            /// success and failure leave input and external state unchanged.
            pub fn saturating_div<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.saturating_div_with(rhs)
            }

            /// Computes raw1 * raw2 / D with half ties away from zero; first rounds,
            /// then returns Overflow if the 64-bit count cannot represent the result.
            /// Does not mutate either input.
            pub fn rounded_mul<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.rounded_mul_with(rhs)
            }

            /// Computes raw1 * D / raw2 with half ties away from zero; first rounds,
            /// then checks 64-bit bounds. Zero divisor returns DivisionByZero,
            /// out-of-range returns Overflow; neither failure mutates inputs.
            pub fn rounded_div<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.rounded_div_with(rhs)
            }

            /// Explicitly converts count to raw * TO / D, truncating toward zero.
            /// TO=0 fails compilation; out-of-range target count returns Overflow.
            /// Does not mutate the source value or any external state.
            pub fn rescale<const TO: u32>(self) -> Result<$name<TO>, ArithmeticError> {
                const { assert!(D > 0 && TO > 0, "denominators must be positive") };
                fixed_inner::$rescale::<D, TO>(self.raw, Rounding::Truncate).map($name::<TO>::from_raw)
            }

            /// Explicitly converts count to raw * TO / D with half ties away from
            /// zero. TO=0 fails compilation; out-of-range count returns Overflow.
            /// Does not mutate the source value or any external state.
            pub fn rescale_rounded<const TO: u32>(self) -> Result<$name<TO>, ArithmeticError> {
                const { assert!(D > 0 && TO > 0, "denominators must be positive") };
                fixed_inner::$rescale::<D, TO>(self.raw, Rounding::HalfAway).map($name::<TO>::from_raw)
            }

            /// Outputs raw / D as a whole integer, truncating toward zero.
            /// Fits the same integer type and never fails or changes the source.
            pub fn to_integer(self) -> $ty {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$to_integer::<D>(self.raw, Rounding::Truncate)
            }

            /// Outputs raw / D as a whole integer with half ties away from zero.
            /// Fits the same integer type and never fails or changes the source.
            pub fn to_integer_rounded(self) -> $ty {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$to_integer::<D>(self.raw, Rounding::HalfAway)
            }
        }

        impl<const D: u32> PartialEq for $name<D> {
            /// Compares counts of the same scale and signedness; has no runtime error.
            /// D=0 fails compilation when comparison is used, without mutating either value.
            fn eq(&self, other: &Self) -> bool {
                const { assert!(D > 0, "denominator must be positive") };
                self.raw == other.raw
            }
        }
        impl<const D: u32> Eq for $name<D> {}
        impl<const D: u32> PartialOrd for $name<D> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl<const D: u32> Ord for $name<D> {
            /// Orders counts of the same scale and signedness without runtime failure.
            /// D=0 fails compilation when comparison is used; both inputs remain unchanged.
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                const { assert!(D > 0, "denominator must be positive") };
                self.raw.cmp(&other.raw)
            }
        }
    };
}

fixed_declarations!(FixedU64, u64, unsigned_integer, unsigned_arithmetic, unsigned_rescale, unsigned_to_integer);
fixed_declarations!(FixedI64, i64, signed_integer, signed_arithmetic, signed_rescale, signed_to_integer);

macro_rules! integer_conversions {
    ($name:ident; $($ty:ty),* $(,)?) => {$ (
        impl<const D: u32> TryFrom<$ty> for $name<D> {
            type Error = ArithmeticError;
            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                let value = i128::try_from(value).map_err(|_| ArithmeticError::Overflow)?;
                Self::try_from_number(value)
            }
        }
    )*};
}
integer_conversions!(FixedU64; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
integer_conversions!(FixedI64; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
