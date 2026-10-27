//! 固定分母的 64 位定点数：数值 = raw / D，最小计数单位为 1/D。
//!
//! [FixedU64] 使用 u64 计数，[FixedI64] 使用 i64 计数。D 是编译期 u32 常量；
//! 公开数值方法与比较要求 D > 0，违反时在相应使用处编译失败。预设 SCALE 常量
//! 表示每一整数单位包含的 raw 数量，也可以直接使用其他正分母。
//!
//! # 转换与基本运算
//! 整数输入乘 D；from_raw/raw 直接存取计数。十进制文本与浮点数的最短往返
//! 十进制字符串按最近计数舍入，恰好半值时远离零；不是先用浮点数乘 D。
//! 文本允许可选符号、小数点和 e/E 指数，不接受空白。Display 最多输出 18 位小数，
//! 在此精度舍入并删除末尾零；格式器的 precision/width 不控制该实现的小数位或填充。
//! 加减只接受相同定点类型；乘除接受实现 [FixedOperand] 的值，并保留左侧类型与 D。
//! checked 乘除与 rescale 向零截断，rounded 版本按半值远离零舍入，
//! saturating 运算将越界计数钳制到目标边界，但饱和除法仍报告零除数。
//! 这些方法返回新值，成功和失败均不修改输入值或外部状态。
//!
//! # 可选功能
//! - math：启用指数、对数、整数幂及 [MathError]。输入与输出数值均为 raw / D。
//!   普通方法可增加计算精度，成功结果相对正确半值远离零舍入最多相差 1 raw；
//!   strict 方法要求上下界证明唯一量化结果，否则预算耗尽返回 PrecisionExhausted。
//!   fast 方法固定向后端请求 128 bit、ToEven 的单值计算，最终 raw 量化仍为半值远离零；
//!   不提供统一 1 raw 误差保证，不升精度、不回退，也不返回 PrecisionExhausted。
//!   后端保护位不受此请求精度约束，不能据此保证固定耗时或加速比例。
//! - serde、fory：分别派生对应序列化实现；raw 是存储字段，D 属于类型参数。
//!
//! 例如 FixedI64::<100>::from_raw(125) 表示 1.25；调用 to_integer 得 1，
//! 而 FixedI64::<100>::from_raw(-150).to_integer_rounded() 得 -2。

// 设计：pi-fixed-scale-int
// 对外声明集中于此；基础算术实现在 fixed_inner.rs。

mod fixed_inner;
#[cfg(feature = "math")]
mod math_inner;
#[cfg(feature = "math")]
pub use math_inner::MathError;
use fixed_inner::{Bound, Op, Rounding};
use std::fmt;
use std::str::FromStr;

/// 可作为乘除法右操作数的定点值或整数。
/// 返回有符号分子和正分母，所表示的数值为两者之比。
/// 内置实现包括两种定点类型及 u8/u16/u32/u64、i8/i16/i32/i64；
/// 自定义实现也应遵守正分母约定。
pub trait FixedOperand {
    /// 拆出 (raw 分子, 正分母)；原生整数的分母为 1。
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

/// 十进制文本无效，或转换后的 raw 计数超出可表示范围。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseFixedError {
    /// 文本语法无效，或指数不能解析为 i32。
    Invalid,
    /// 十进制转换溢出，或舍入后的计数超出目标 raw 类型范围。
    Overflow,
}

impl fmt::Display for ParseFixedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ParseFixedError {}

/// 十进制一位比例，每整数单位 10 raw。
pub const SCALE_10: u32 = 10;
/// 十进制两位比例，每整数单位 100 raw。
pub const SCALE_100: u32 = 100;
/// 十进制三位比例，每整数单位 1,000 raw。
pub const SCALE_1000: u32 = 1_000;
/// 十进制四位比例，每整数单位 10,000 raw。
pub const SCALE_10000: u32 = 10_000;
/// 十进制五位比例，每整数单位 100,000 raw。
pub const SCALE_100000: u32 = 100_000;
/// 十进制六位比例，每整数单位 1,000,000 raw。
pub const SCALE_1000000: u32 = 1_000_000;
/// 十进制七位比例，每整数单位 10,000,000 raw。
pub const SCALE_10000000: u32 = 10_000_000;
/// 十进制八位比例，每整数单位 100,000,000 raw。
pub const SCALE_100000000: u32 = 100_000_000;
/// 复合分母比例，每整数单位 75,600 raw。
pub const SCALE_75600: u32 = 75_600; // 2**4 * 3**3 * 5**2 * 7
/// 复合分母比例，每整数单位 324,000 raw。
pub const SCALE_324000: u32 = 324_000; // 2**5 * 3**4 * 5**3
/// 复合分母比例，每整数单位 1,371,600 raw。
pub const SCALE_1371600: u32 = 1_371_600; // 2**4 * 3**3 * 5**2 * 127
/// 复合分母比例，每整数单位 2,268,000 raw。
pub const SCALE_2268000: u32 = 2_268_000; // 2**5 * 3**4 * 5**3 * 7
/// 复合分母比例，每整数单位 9,601,200 raw。
pub const SCALE_9601200: u32 = 9_601_200; // 2**4 * 3**3 * 5**2 * 7 * 127

/// 基本算术或数值转换失败；返回错误不修改输入值或外部状态。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticError {
    /// 中间转换或最终 raw 计数不能在相应范围内表示。
    Overflow,
    /// 除法右操作数的 raw 为零。
    DivisionByZero,
}

macro_rules! fixed_declarations {
    ($name:ident, $ty:ty, $integer:ident, $arithmetic:ident, $rescale:ident, $to_integer:ident) => {
        /// 定点值为 raw / D，raw 使用类型名称对应的 64 位整数域。
        /// 调用公开方法或进行比较时，D 必须为正的 u32 常量；D=0 在相应使用处编译失败。
        /// 该 Copy 值拥有私有计数，不借用数据；方法返回新值，不修改原值。
        /// 加减要求同一类型；乘除可接受不同符号类别或分母，结果保留左操作数的类型与比例。
        ///
        /// 十进制解析支持可选正负号、小数点及 e/E 指数，不接受空白或分隔符；
        /// 按 raw 单位四舍五入，恰好半值时远离零。Display 最多生成 18 位小数，
        /// 在该位数按半值远离零舍入后去掉末尾零；不使用格式器指定的小数精度或宽度。
        /// 启用 serde/fory 时派生相应序列化实现，存储字段为 raw，分母由类型参数确定。
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
            /// 直接存入以 1/D 为单位的 raw 计数，不进行缩放。
            /// 无运行时失败；使用 D=0 会编译失败。不修改输入或外部状态。
            pub fn from_raw(raw: $ty) -> Self {
                const { assert!(D > 0, "denominator must be positive") };
                Self { raw }
            }

            /// 读取以 1/D 为单位的 raw 计数，不进行转换或修改。
            /// 无运行时失败；使用 D=0 会编译失败。
            pub fn raw(self) -> $ty {
                const { assert!(D > 0, "denominator must be positive") };
                self.raw
            }

            /// 将整数转换为 raw = value * D。超出目标计数范围返回 Overflow；
            /// 输入和外部状态保持不变。
            pub fn try_from_integer(value: $ty) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$integer::<D>(value).map(Self::from_raw)
            }

            /// 将有限 f64 的 value.to_string() 最短往返十进制字符串转换为 1/D 单位的 raw。
            /// 按十进制半值远离零舍入，并非对精确二进制浮点数直接缩放。
            /// 非有限输入、转换失败或舍入后计数越界均返回 Overflow。
            /// D=0 会编译失败；输入及外部状态保持不变。
            pub fn try_from_f64(value: f64) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                if !value.is_finite() { return Err(ArithmeticError::Overflow); }
                let count = fixed_inner::parse_decimal(&value.to_string(), D)
                    .map_err(|_| ArithmeticError::Overflow)?;
                <$ty>::try_from(count).map(Self::from_raw).map_err(|_| ArithmeticError::Overflow)
            }

            /// 将可通过 `Into<i128>` 转换的整数无损缩放到此比例（raw = value * D）。
            /// 乘法溢出或目标计数不可表示时返回 Overflow；D=0 会编译失败。
            /// 成功或失败均不修改输入和外部状态。
            pub fn try_from_number<I: Into<i128>>(value: I) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let count = value.into().checked_mul(i128::from(D)).ok_or(ArithmeticError::Overflow)?;
                <$ty>::try_from(count).map(Self::from_raw).map_err(|_| ArithmeticError::Overflow)
            }

            /// 将有限 f32 的 value.to_string() 最短往返十进制字符串转换为 1/D 单位的 raw，
            /// 不先拓宽为 f64。按十进制半值远离零舍入，而非直接缩放精确二进制值。
            /// 非有限输入、转换失败或舍入后计数越界均返回 Overflow。
            /// D=0 会编译失败；输入及外部状态保持不变。
            pub fn try_from_f32(value: f32) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                if !value.is_finite() { return Err(ArithmeticError::Overflow); }
                let count = fixed_inner::parse_decimal(&value.to_string(), D)
                    .map_err(|_| ArithmeticError::Overflow)?;
                <$ty>::try_from(count).map(Self::from_raw).map_err(|_| ArithmeticError::Overflow)
            }

            /// 与不同符号类别、不同分母的定点值或原生整数相乘。
            /// 结果保留左操作数类型与分母；raw = lhs_raw * rhs_raw / rhs_denominator，向零截断。
            /// 最终计数越界返回 Overflow；不修改输入或外部状态。
            pub fn checked_mul_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Mul, Rounding::Truncate, Bound::Checked, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// 除以另一整数类型/比例的定点值或原生整数，保留左操作数类型与分母。
            /// raw = lhs_raw * rhs_denominator / rhs_raw，向零截断。
            /// 零除数返回 DivisionByZero，越界返回 Overflow；输入和外部状态不变。
            pub fn checked_div_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Div, Rounding::Truncate, Bound::Checked, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// 相乘后按最近 raw 计数舍入，恰好半值时远离零；保留左操作数类型与分母。
            /// 舍入后越界返回 Overflow；不修改输入或外部状态。
            pub fn rounded_mul_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Mul, Rounding::HalfAway, Bound::Checked, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// 相除后按最近 raw 计数舍入，恰好半值时远离零；保留左操作数类型与分母。
            /// 零除数返回 DivisionByZero，舍入后越界返回 Overflow；输入和外部状态不变。
            pub fn rounded_div_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Div, Rounding::HalfAway, Bound::Checked, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// 相乘并向零截断，将越界计数钳制到结果类型上下界；保留左操作数比例。
            /// 对内置操作数不返回错误，不修改输入或外部状态。
            pub fn saturating_mul_with<R: FixedOperand>(self, rhs: R) -> Self {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                Self::from_raw(fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Mul, Rounding::Truncate, Bound::Saturate, <$ty>::MIN as i128, <$ty>::MAX as u128).expect("multiplication cannot divide by zero"))
            }

            /// 相除并向零截断，将越界计数钳制到结果类型上下界；保留左操作数比例。
            /// 零除数仍返回 DivisionByZero；输入和外部状态保持不变。
            pub fn saturating_div_with<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                let (raw, denominator) = rhs.parts();
                fixed_inner::mixed_arithmetic::<$ty>(self.raw as i128, raw, denominator, Op::Div, Rounding::Truncate, Bound::Saturate, <$ty>::MIN as i128, <$ty>::MAX as u128).map(Self::from_raw)
            }

            /// 相加以 1/D 为单位的 raw 计数。越界返回 Overflow；
            /// 成功或失败均不修改任一输入或外部状态。
            pub fn checked_add(self, rhs: Self) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$arithmetic::<D>(self.raw, rhs.raw, Op::Add, Rounding::Truncate, Bound::Checked).map(Self::from_raw)
            }

            /// 相减以 1/D 为单位的 raw 计数。越界返回 Overflow；
            /// 成功或失败均不修改任一输入或外部状态。
            pub fn checked_sub(self, rhs: Self) -> Result<Self, ArithmeticError> {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$arithmetic::<D>(self.raw, rhs.raw, Op::Sub, Rounding::Truncate, Bound::Checked).map(Self::from_raw)
            }

            /// 相乘后在左操作数比例下向零截断。
            /// 最终计数越界返回 Overflow；输入和外部状态保持不变。
            pub fn checked_mul<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.checked_mul_with(rhs)
            }

            /// 相除后在左操作数比例下向零截断。
            /// 零除数返回 DivisionByZero，计数越界返回 Overflow。
            /// 任一错误均不修改输入或外部状态。
            pub fn checked_div<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.checked_div_with(rhs)
            }

            /// 相加 raw 计数，超过 MAX 或低于 MIN 时钳制到边界（u64 的 MIN 为零）。
            /// 不返回错误；输入和外部状态保持不变。
            pub fn saturating_add(self, rhs: Self) -> Self {
                const { assert!(D > 0, "denominator must be positive") };
                Self::from_raw(fixed_inner::$arithmetic::<D>(self.raw, rhs.raw, Op::Add, Rounding::Truncate, Bound::Saturate).expect("addition cannot divide by zero"))
            }

            /// 相减 raw 计数，超过 MAX 或低于 MIN 时钳制到边界（u64 的 MIN 为零）。
            /// 不返回错误；输入和外部状态保持不变。
            pub fn saturating_sub(self, rhs: Self) -> Self {
                const { assert!(D > 0, "denominator must be positive") };
                Self::from_raw(fixed_inner::$arithmetic::<D>(self.raw, rhs.raw, Op::Sub, Rounding::Truncate, Bound::Saturate).expect("subtraction cannot divide by zero"))
            }

            /// 计算 lhs_raw * rhs_raw / rhs_denominator（同分母时为 raw1 * raw2 / D），
            /// 向零截断后将最终计数钳制到 64 位边界。内置操作数下不返回错误，不修改输入。
            pub fn saturating_mul<R: FixedOperand>(self, rhs: R) -> Self {
                self.saturating_mul_with(rhs)
            }

            /// 计算 lhs_raw * rhs_denominator / rhs_raw（同分母时为 raw1 * D / raw2），
            /// 向零截断后将最终计数钳制到 64 位边界。零除数返回 DivisionByZero；
            /// 成功或失败均不修改输入或外部状态。
            pub fn saturating_div<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.saturating_div_with(rhs)
            }

            /// 计算 lhs_raw * rhs_raw / rhs_denominator（同分母时为 raw1 * raw2 / D），
            /// 先按半值远离零舍入，再检查 64 位范围；不可表示则返回 Overflow。
            /// 不修改任一输入或外部状态。
            pub fn rounded_mul<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.rounded_mul_with(rhs)
            }

            /// 计算 lhs_raw * rhs_denominator / rhs_raw（同分母时为 raw1 * D / raw2），
            /// 先按半值远离零舍入，再检查 64 位范围。零除数返回 DivisionByZero，
            /// 越界返回 Overflow；任一失败均不修改输入或外部状态。
            pub fn rounded_div<R: FixedOperand>(self, rhs: R) -> Result<Self, ArithmeticError> {
                self.rounded_div_with(rhs)
            }

            /// 显式换算为 raw * TO / D，并向零截断。
            /// D=0 或 TO=0 会编译失败；目标计数越界返回 Overflow。
            /// 不修改源值或外部状态。
            pub fn rescale<const TO: u32>(self) -> Result<$name<TO>, ArithmeticError> {
                const { assert!(D > 0 && TO > 0, "denominators must be positive") };
                fixed_inner::$rescale::<D, TO>(self.raw, Rounding::Truncate).map($name::<TO>::from_raw)
            }

            /// 显式换算为 raw * TO / D，恰好半值时远离零舍入。
            /// D=0 或 TO=0 会编译失败；目标计数越界返回 Overflow。
            /// 不修改源值或外部状态。
            pub fn rescale_rounded<const TO: u32>(self) -> Result<$name<TO>, ArithmeticError> {
                const { assert!(D > 0 && TO > 0, "denominators must be positive") };
                fixed_inner::$rescale::<D, TO>(self.raw, Rounding::HalfAway).map($name::<TO>::from_raw)
            }

            /// 输出 raw / D 对应的整数，向零截断。
            /// 结果适合相同整数类型，不会运行时失败，也不修改源值。
            pub fn to_integer(self) -> $ty {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$to_integer::<D>(self.raw, Rounding::Truncate)
            }

            /// 输出 raw / D 对应的整数，恰好半值时远离零舍入。
            /// 结果适合相同整数类型，不会运行时失败，也不修改源值。
            pub fn to_integer_rounded(self) -> $ty {
                const { assert!(D > 0, "denominator must be positive") };
                fixed_inner::$to_integer::<D>(self.raw, Rounding::HalfAway)
            }
        }

        impl<const D: u32> PartialEq for $name<D> {
            /// 比较同一比例、同一符号类别的计数，无运行时错误。
            /// 使用 D=0 的比较会编译失败，不修改任一值。
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
            /// 对同一比例、同一符号类别的计数排序，无运行时失败。
            /// 使用 D=0 的比较会编译失败，两个输入保持不变。
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
        // 原生整数先尝试进入 i128，再乘 D 并检查目标 raw 范围；
        // 即使目标为无符号类型，超过 i128 的 u128 也在第一步返回 Overflow。
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
