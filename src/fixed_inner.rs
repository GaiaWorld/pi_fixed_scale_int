// 设计：pi-fixed-scale-int

use crate::ArithmeticError;
use crate::ParseFixedError;
use std::fmt;

pub(crate) fn parse_decimal(text: &str, denominator: u32) -> Result<i128, ParseFixedError> {
    let (base, power) = match text.split_once(['e', 'E']) {
        Some((base, exponent)) => (base, exponent.parse::<i32>().map_err(|_| ParseFixedError::Invalid)?),
        None => (text, 0),
    };
    let (negative, base) = match base.as_bytes().first() {
        Some(b'-') => (true, &base[1..]),
        Some(b'+') => (false, &base[1..]),
        _ => (false, base),
    };
    let mut digits = String::new();
    let mut fraction = 0i64;
    let mut point = false;
    for ch in base.bytes() {
        if ch == b'.' && !point { point = true; continue; }
        if !ch.is_ascii_digit() { return Err(ParseFixedError::Invalid); }
        if point { fraction += 1; }
        digits.push(ch as char);
        if digits.len() > 10_000 { return Err(ParseFixedError::Overflow); }
    }
    if digits.is_empty() { return Err(ParseFixedError::Invalid); }
    let digits = digits.trim_start_matches('0');
    if digits.is_empty() { return Ok(0); }
    // Multiply decimal digits by D before removing fractional digits: this
    // preserves ties exactly, including values smaller than one raw unit.
    let mut product = vec![0u8; digits.len() + 12];
    let mut carry = 0u64;
    for (index, digit) in digits.bytes().rev().enumerate() {
        let value = u64::from(digit - b'0') * u64::from(denominator) + carry;
        product[index] = (value % 10) as u8;
        carry = value / 10;
    }
    let mut length = digits.len();
    while carry > 0 {
        product[length] = (carry % 10) as u8;
        carry /= 10;
        length += 1;
    }
    while length > 1 && product[length - 1] == 0 { length -= 1; }
    let shift = i64::from(power) - fraction;
    if shift < -(length as i64) { return Ok(0); }
    let output_digits = length as i64 + shift;
    if output_digits > 39 { return Err(ParseFixedError::Overflow); }
    if output_digits < 0 { return Ok(0); }
    let mut result = 0u128;
    if shift >= 0 {
        for &digit in product[..length].iter().rev() {
            result = result.checked_mul(10).and_then(|n| n.checked_add(u128::from(digit)))
                .ok_or(ParseFixedError::Overflow)?;
        }
        if shift > 0 {
            let factor = 10u128.checked_pow(shift as u32).ok_or(ParseFixedError::Overflow)?;
            result = result.checked_mul(factor).ok_or(ParseFixedError::Overflow)?;
        }
    } else {
        let dropped = (-shift) as usize;
        for &digit in product[dropped.min(length)..length].iter().rev() {
            result = result.checked_mul(10).and_then(|n| n.checked_add(u128::from(digit)))
                .ok_or(ParseFixedError::Overflow)?;
        }
        if dropped > 0 && dropped <= length && product[dropped - 1] >= 5 {
            result = result.checked_add(1).ok_or(ParseFixedError::Overflow)?;
        }
    }
    let limit = if negative { (i128::MAX as u128) + 1 } else { i128::MAX as u128 };
    if result > limit { return Err(ParseFixedError::Overflow); }
    if negative && result == limit { Ok(i128::MIN) }
    else if negative { Ok(-(result as i128)) } else { Ok(result as i128) }
}

pub(crate) fn display_decimal(raw: i128, denominator: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut whole = raw.abs() / i128::from(denominator);
    let mut remainder = raw.abs() % i128::from(denominator);
    let mut fraction = [0u8; 18];
    let mut len = 0;
    while remainder != 0 && len < fraction.len() {
        remainder *= 10;
        fraction[len] = (remainder / i128::from(denominator)) as u8;
        remainder %= i128::from(denominator);
        len += 1;
    }
    if remainder * 2 >= i128::from(denominator) {
        let mut carry = true;
        for digit in fraction[..len].iter_mut().rev() {
            if !carry { break; }
            if *digit == 9 { *digit = 0; } else { *digit += 1; carry = false; }
        }
        if carry { whole += 1; }
    }
    while len > 0 && fraction[len - 1] == 0 { len -= 1; }
    if raw < 0 && (whole != 0 || len != 0) { f.write_str("-")?; }
    write!(f, "{whole}")?;
    if len != 0 {
        f.write_str(".")?;
        for digit in &fraction[..len] { write!(f, "{digit}")?; }
    }
    Ok(())
}

pub(crate) fn mixed_arithmetic<T: TryFrom<i128>>(
    lhs: i128, rhs: i128, rhs_denominator: u32, op: Op, rounding: Rounding, bound: Bound,
    min: i128, max: u128,
) -> Result<T, ArithmeticError> {
    if rhs_denominator == 0 { return Err(ArithmeticError::Overflow); }
    if matches!(op, Op::Div) && rhs == 0 { return Err(ArithmeticError::DivisionByZero); }
    let positive = (lhs < 0) == (rhs < 0);
    let left = lhs.unsigned_abs();
    let right = rhs.unsigned_abs();
    let (factor, divisor) = if matches!(op, Op::Mul) {
        (right, u128::from(rhs_denominator))
    } else {
        (u128::from(rhs_denominator), right)
    };
    // A 64-bit raw count times a 64-bit count (or u32 denominator) fits u128.
    let numerator = left.checked_mul(factor).ok_or(ArithmeticError::Overflow)?;
    let rounded = (numerator / divisor).checked_add({
        let remainder = numerator % divisor;
        let increment = matches!(rounding, Rounding::HalfAway) && remainder >= divisor - remainder;
        u128::from(increment)
    });
    let lower = if min < 0 { min.unsigned_abs() } else { min as u128 };
    let limit = if positive { max } else { lower };
    let magnitude = match rounded {
        Some(value) if value <= limit => value,
        _ if matches!(bound, Bound::Saturate) => limit,
        _ => return Err(ArithmeticError::Overflow),
    };
    let signed = if positive { magnitude as i128 }
        else if magnitude == (i128::MAX as u128) + 1 { i128::MIN }
        else { -(magnitude as i128) };
    T::try_from(signed).map_err(|_| ArithmeticError::Overflow)
}

#[derive(Clone, Copy)]
pub(crate) enum Op { Add, Sub, Mul, Div }
#[derive(Clone, Copy)]
pub(crate) enum Rounding { Truncate, HalfAway }
#[derive(Clone, Copy)]
pub(crate) enum Bound { Checked, Saturate }

macro_rules! inner_stubs {
    ($integer:ident, $arithmetic:ident, $rescale:ident, $to_integer:ident, $ty:ty, $wide:ty) => {
        pub(crate) fn $integer<const D: u32>(value: $ty) -> Result<$ty, ArithmeticError> {
            let count = (value as $wide) * (D as $wide);
            <$ty>::try_from(count).map_err(|_| ArithmeticError::Overflow)
        }

        pub(crate) fn $arithmetic<const D: u32>(
            _lhs: $ty, _rhs: $ty, _op: Op, _round: Rounding, _bound: Bound,
        ) -> Result<$ty, ArithmeticError> {
            let count = match _op {
                Op::Add => (_lhs as $wide) + (_rhs as $wide),
                Op::Sub => match (_lhs as $wide).checked_sub(_rhs as $wide) {
                    Some(count) => count,
                    None => return match _bound {
                        Bound::Checked => Err(ArithmeticError::Overflow),
                        Bound::Saturate => Ok(<$ty>::MIN),
                    },
                },
                Op::Mul | Op::Div => {
                    let (numerator, divisor) = match _op {
                        Op::Mul => ((_lhs as $wide) * (_rhs as $wide), D as $wide),
                        Op::Div => {
                            if _rhs == 0 {
                                return Err(ArithmeticError::DivisionByZero);
                            }
                            ((_lhs as $wide) * (D as $wide), _rhs as $wide)
                        }
                        _ => unreachable!("only multiplication and division reach this branch"),
                    };
                    let mut quotient = numerator / divisor;
                    let remainder = numerator % divisor;
                    // The remainder and divisor fit in i128 even for the u128 path.
                    let rem = remainder as i128;
                    let den = divisor as i128;
                    if matches!(_round, Rounding::HalfAway) && rem.abs() * 2 >= den.abs() {
                        quotient = if rem.is_negative() != den.is_negative() {
                            quotient - 1
                        } else {
                            quotient + 1
                        };
                    }
                    quotient
                }
            };

            match _bound {
                Bound::Checked => <$ty>::try_from(count).map_err(|_| ArithmeticError::Overflow),
                Bound::Saturate => Ok(count.clamp(<$ty>::MIN as $wide, <$ty>::MAX as $wide) as $ty),
            }
        }

        pub(crate) fn $rescale<const D: u32, const TO: u32>(
            _raw: $ty, _round: Rounding,
        ) -> Result<$ty, ArithmeticError> {
            let numerator = (_raw as $wide) * (TO as $wide);
            let divisor = D as $wide;
            let mut quotient = numerator / divisor;
            let remainder = numerator % divisor;
            if matches!(_round, Rounding::HalfAway)
                && (remainder as i128).abs() * 2 >= D as i128
            {
                quotient = if (remainder as i128) < 0 { quotient - 1 } else { quotient + 1 };
            }
            <$ty>::try_from(quotient).map_err(|_| ArithmeticError::Overflow)
        }

        pub(crate) fn $to_integer<const D: u32>(_raw: $ty, _round: Rounding) -> $ty {
            let numerator = _raw as $wide;
            let divisor = D as $wide;
            let mut quotient = numerator / divisor;
            let remainder = numerator % divisor;
            if matches!(_round, Rounding::HalfAway)
                && (remainder as i128).abs() * 2 >= D as i128
            {
                quotient = if (remainder as i128) < 0 { quotient - 1 } else { quotient + 1 };
            }
            quotient as $ty
        }
    };
}

inner_stubs!(unsigned_integer, unsigned_arithmetic, unsigned_rescale, unsigned_to_integer, u64, u128);
inner_stubs!(signed_integer, signed_arithmetic, signed_rescale, signed_to_integer, i64, i128);
