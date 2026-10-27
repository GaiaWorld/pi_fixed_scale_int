use crate::{FixedI64, FixedU64};
use astro_float::{BigFloat, Consts, RoundingMode as R};

// 观测及可注入区间接缝仅存在于 cfg(test) 编译；线程局部存储避免并行用例互相污染。
// 非测试构建移除事件、队列及相应调用，不承担这些接缝的存储或运行时开销。
#[cfg(test)]
#[derive(Debug)]
enum MathTraceEvent {
    BackendRequest {
        operation: &'static str,
        precision: usize,
        rounding: R,
    },
    AdaptiveEntry {
        operation: &'static str,
    },
    PrecisionAttempt {
        precision: usize,
    },
    PrecisionResult {
        result: Result<i128, MathError>,
    },
}

#[cfg(test)]
std::thread_local! {
    static MATH_TRACE: std::cell::RefCell<Vec<MathTraceEvent>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}

// 仅供测试按精度提供对数分母区间；线程隔离，调用前后由测试清空。
#[cfg(test)]
std::thread_local! {
    static LOG_DENOMINATOR_TEST_BOUNDS: std::cell::RefCell<std::collections::VecDeque<(usize, Interval)>> = const {
        std::cell::RefCell::new(std::collections::VecDeque::new())
    };
}

#[cfg(test)]
fn reset_math_trace() {
    MATH_TRACE.with(|trace| trace.borrow_mut().clear());
}

#[cfg(test)]
fn take_math_trace() -> Vec<MathTraceEvent> {
    MATH_TRACE.with(|trace| std::mem::take(&mut *trace.borrow_mut()))
}

// 在实际调用点读取同一组精度/舍入变量，不以常量日志代替实际请求。
// 非测试展开只保留原表达式，不产生观测调用或运行时分支。
macro_rules! fast_request {
    ($operation:literal, $precision:ident, $rounding:ident, $request:expr) => {{
        #[cfg(test)]
        MATH_TRACE.with(|trace| {
            trace.borrow_mut().push(MathTraceEvent::BackendRequest {
                operation: $operation,
                precision: $precision,
                rounding: $rounding,
            });
        });
        $request
    }};
}

/// 可选 math 功能中高精度数学运算的错误。普通、严格、快速档的保证各见对应方法。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MathError {
    /// 对数输入非正、底数不合法，或零的负整数次幂。
    Domain,
    /// 普通/严格档：正确半值远离零舍入后的 raw 越界；
    /// 快速档：量化近似 raw 越界，或独立数学界已证明越界。
    Overflow,
    /// 精度预算内无法证明所要求的舍入结果或误差界；快速档不返回此错误。
    PrecisionExhausted,
    /// 后端错误、NaN、没有独立界支持的无穷，或无效区间。
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

// lo/hi 以向下/向上定向舍入包围真值；evaluate 的最终区间单位为 raw。
// 有限、无后端错误且 lo <= hi 才是有效区间，不能仅检查某个最终聚合值。
struct Interval {
    lo: BigFloat,
    hi: BigFloat,
}

fn validate_endpoint(value: &BigFloat) -> Result<(), MathError> {
    if value.err().is_some() || value.is_nan() || value.is_inf() {
        Err(MathError::ComputationFailed)
    } else {
        Ok(())
    }
}

impl Interval {
    fn validate(&self) -> Result<(), MathError> {
        validate_endpoint(&self.lo)?;
        validate_endpoint(&self.hi)?;
        match self.lo.cmp(&self.hi) {
            Some(order) if order <= 0 => Ok(()),
            _ => Err(MathError::ComputationFailed),
        }
    }

    fn checked(self) -> Result<Self, MathError> {
        self.validate()?;
        Ok(self)
    }

    // 仅用于已知数学真值为正的区间。下端点为零说明界尚不够紧，
    // 应升精度重试，不能把零直接送入倒数；负下界则判为计算失败。
    fn require_positive(&self, p: usize) -> Result<(), MathError> {
        self.validate()?;
        match self.lo.cmp(&integer(0, p)) {
            Some(order) if order > 0 => Ok(()),
            Some(0) => Err(MathError::PrecisionExhausted),
            _ => Err(MathError::ComputationFailed),
        }
    }
}

fn integer(n: i128, p: usize) -> BigFloat {
    BigFloat::from_i128(n, p)
}

fn ratio(n: i128, d: u32, p: usize) -> Result<Interval, MathError> {
    let n = integer(n, p);
    let d = BigFloat::from_u32(d, p);
    Interval {
        lo: n.div(&d, p, R::Down),
        hi: n.div(&d, p, R::Up),
    }
    .checked()
}

fn divide(a: Interval, b: Interval, p: usize) -> Result<Interval, MathError> {
    a.validate()?;
    b.validate()?;
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
    // 分母不跨零后，分别向外计算四角，再逐一验证全部候选值。
    // 必须先验证再 min/max，否则聚合可能掩盖某个角点的 NaN/无穷或后端错误。
    for value in lows.iter().chain(highs.iter()) {
        validate_endpoint(value)?;
    }
    #[cfg(test)]
    MATH_TRACE.with(|trace| trace.borrow_mut().push(MathTraceEvent::AdaptiveEntry { operation: "divide.aggregate" }));
    let lo = lows
        .into_iter()
        .reduce(|a, b| a.min(&b))
        .ok_or(MathError::ComputationFailed)?;
    let hi = highs
        .into_iter()
        .reduce(|a, b| a.max(&b))
        .ok_or(MathError::ComputationFailed)?;
    Interval { lo, hi }.checked()
}

fn evaluate(
    raw: i128,
    d: u32,
    function: Function,
    p: usize,
    cc: &mut Consts,
) -> Result<Interval, MathError> {
    #[cfg(test)]
    MATH_TRACE.with(|trace| {
        trace.borrow_mut().push(MathTraceEvent::AdaptiveEntry {
            operation: "evaluate",
        });
    });
    let x = ratio(raw, d, p)?;
    if matches!(
        function,
        Function::Ln | Function::Log2 | Function::Log10 | Function::Log(_)
    ) {
        x.require_positive(p)?;
    }
    let result = match function {
        Function::Exp => Interval {
            lo: x.lo.exp(p, R::Down, cc),
            hi: x.hi.exp(p, R::Up, cc),
        },
        Function::Exp2 => {
            let two = integer(2, p);
            if raw % i128::from(d) == 0 {
                // 整数指数的结果是精确二进制数。通用 pow 经 exp(n*ln(2))
                // 逼近它时，后端的定向舍入确认可能持续升精度而无法结束。
                // 改用整数幂；compute 已将指数限制在 [-100, 100]，
                // 因此绝对值转换安全，正幂及倒数均远离后端指数范围边界。
                let n = raw / i128::from(d);
                let power = Interval {
                    lo: two.powi(n.unsigned_abs() as usize, p, R::Down),
                    hi: two.powi(n.unsigned_abs() as usize, p, R::Up),
                }
                .checked()?;
                if n < 0 {
                    // 倒数递减，先验证正端点，再交换上下界并向外舍入。
                    power.require_positive(p)?;
                    Interval {
                        lo: power.hi.reciprocal(p, R::Down),
                        hi: power.lo.reciprocal(p, R::Up),
                    }
                    .checked()?
                } else {
                    power
                }
            } else {
                Interval {
                    lo: two.pow(&x.lo, p, R::Down, cc),
                    hi: two.pow(&x.hi, p, R::Up, cc),
                }
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
            let b = ratio(base, d, p)?;
            b.require_positive(p)?;
            let numerator = Interval {
                lo: x.lo.ln(p, R::Down, cc),
                hi: x.hi.ln(p, R::Up, cc),
            };
            let denominator = Interval {
                lo: b.lo.ln(p, R::Down, cc),
                hi: b.hi.ln(p, R::Up, cc),
            }
            .checked()?;
            // 测试只能放宽真实区间，不能伪造错误或排除真实数学值。
            // 非测试构建不消费队列，仍将原有有向对数区间传给真实除法。
            #[cfg(test)]
            let denominator = LOG_DENOMINATOR_TEST_BOUNDS.with(|queue| -> Result<Interval, MathError> {
                let mut queue = queue.borrow_mut();
                if queue.front().is_some_and(|(precision, _)| *precision == p) {
                    // 已检查队首存在且精度匹配，独占借用期间不会被移除。
                    let (_, widened) = queue.pop_front().expect("front checked");
                    widened.validate()?;
                    if !widened.lo.cmp(&denominator.lo).is_some_and(|order| order <= 0)
                        || !widened.hi.cmp(&denominator.hi).is_some_and(|order| order >= 0)
                    {
                        return Err(MathError::ComputationFailed);
                    }
                    Ok(widened)
                } else {
                    Ok(denominator)
                }
            })?;
            divide(numerator, denominator, p)?
        }
        Function::Pow(n) => {
            if n == 0 {
                // 零次幂为 1（包括 0^0），这里已返回 D raw，不再经过下方乘 D。
                return Interval {
                    lo: integer(d as i128, p),
                    hi: integer(d as i128, p),
                }
                .checked();
            }
            // 在非负 |x| 上求正整数幂以使用单调性，最后才恢复奇次幂的符号。
            // raw 源自 64 位计数，提升后的 abs 安全；unsigned_abs 可处理 i32::MIN。
            let a = ratio(raw.abs(), d, p)?;
            let mut power = Interval {
                lo: a.lo.powi(n.unsigned_abs() as usize, p, R::Down),
                hi: a.hi.powi(n.unsigned_abs() as usize, p, R::Up),
            }
            .checked()?;
            if n < 0 {
                // 先验证正幂区间；倒数递减，交换端点并继续向外舍入。
                // 正幂失败必须传播，不能把无穷取倒数伪装成精确零。
                power.require_positive(p)?;
                #[cfg(test)]
                MATH_TRACE.with(|trace| trace.borrow_mut().push(MathTraceEvent::AdaptiveEntry { operation: "power.reciprocal" }));
                power = Interval {
                    lo: power.hi.reciprocal(p, R::Down),
                    hi: power.lo.reciprocal(p, R::Up),
                }
                .checked()?;
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
    result.validate()?;
    let scale = BigFloat::from_u32(d, p);
    Interval {
        lo: result.lo.mul(&scale, p, R::Down),
        hi: result.hi.mul(&scale, p, R::Up),
    }
    .checked()
}

// 直接量化二进制值，不经十进制格式化或 f64。先二分找到幅值整数部分，
// 再与精确半整数比较；达到半值就增大幅值，最后恢复符号，即半值远离零。
// 构造 half 虽使用 ToEven，但该半整数可精确表示，不会把最终量化改成半值取偶。
// ±2^65 哨兵超出全部支持的 64 位 raw 范围，仅供上层判断越界。
fn quantize(value: &BigFloat, p: usize) -> Result<i128, MathError> {
    validate_endpoint(value)?;
    const LIMIT: i128 = 1i128 << 65;
    if value.cmp(&integer(LIMIT, p)).is_some_and(|n| n >= 0) {
        return Ok(LIMIT);
    }
    if value.cmp(&integer(-LIMIT, p)).is_some_and(|n| n <= 0) {
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
    let rounding = R::ToEven;
    let half = fast_request!(
        "quantize.half.div",
        p,
        rounding,
        integer(2 * low + 1, p).div(&integer(2, p), p, rounding)
    );
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
    #[cfg(test)]
    MATH_TRACE.with(|trace| {
        trace.borrow_mut().push(MathTraceEvent::AdaptiveEntry {
            operation: "compute",
        });
    });
    compute_with_budget(
        raw,
        d,
        function,
        strict,
        min,
        max,
        if strict { 192 } else { 128 },
        12288,
    )
}

// 与生产入口共享唯一的实际循环；预算只控制起始精度与停止条件。
// 至少 128 bit 使输入整数及量化半值阈值始终精确，不能用过低预算
// 破坏区间证明的前提。测试应选择更接近舍入边界的输入驱动升精度。
fn compute_with_budget(
    raw: i128,
    d: u32,
    function: Function,
    strict: bool,
    min: i128,
    max: i128,
    start_precision: usize,
    max_precision: usize,
) -> Result<i128, MathError> {
    // 统一记录最终结果，包含域错误、独立界早退与后端错误，避免
    // 在每个 return 点分别插桩而遗漏错误传播路径。
    let result = (|| {
        match function {
            Function::Ln | Function::Log2 | Function::Log10 if raw <= 0 => {
                return Err(MathError::Domain);
            }
            Function::Log(base) if raw <= 0 || base <= 0 || base == i128::from(d) => {
                return Err(MathError::Domain);
            }
            Function::Pow(n) if raw == 0 && n < 0 => return Err(MathError::Domain),
            // x>100 时 e^x 与 2^x 已超出全部 64 位 raw 范围；
            // x<-100 时即使乘 u32::MAX 也不足半个 raw，可独立证明舍入到零。
            Function::Exp | Function::Exp2 if raw > 100 * i128::from(d) => {
                return Err(MathError::Overflow);
            }
            Function::Exp | Function::Exp2 if raw < -100 * i128::from(d) => return Ok(0),
            _ => {}
        }
        let mut cc = Consts::new().map_err(|_| MathError::ComputationFailed)?;
        // 普通入口从 128 bit、严格入口从 192 bit 开始，预算上限 12288 bit；
        // 每次翻倍，只有下一精度仍在预算内才执行，后端硬错误直接传播。
        let mut p = start_precision.max(128);
        while p <= max_precision {
            #[cfg(test)]
            MATH_TRACE.with(|trace| {
                trace
                    .borrow_mut()
                    .push(MathTraceEvent::PrecisionAttempt { precision: p });
            });
            let bounds = match evaluate(raw, d, function, p, &mut cc) {
                Err(MathError::PrecisionExhausted) => {
                    let Some(next) = p.checked_mul(2) else { break };
                    p = next;
                    continue;
                }
                other => other?,
            };
            // 量化单调：端点量化一致即证明唯一结果；普通档还允许端点相差 1 raw，
            // 且两者均在目标范围内，此时返回下端点，误差最多 1 raw。
            // 区间完全越界才报告 Overflow；跨越边界但未能证明则继续升精度。
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
            let Some(next) = p.checked_mul(2) else { break };
            p = next;
        }
        Err(MathError::PrecisionExhausted)
    })();
    #[cfg(test)]
    MATH_TRACE.with(|trace| {
        trace
            .borrow_mut()
            .push(MathTraceEvent::PrecisionResult { result });
    });
    result
}

macro_rules! unary {
    ($name:ident,$strict:ident,$kind:ident,$description:literal) => {
        #[doc = $description]
        /// 计算对应函数：exp 为自然指数，exp2 为二次指数，ln/log2/log10 为相应底数的对数。
        /// 高精度成功结果与正确半值远离零舍入结果最多相差 1 raw（即数值 1/D）。
        /// 保留计数类别与分母，数值为 raw / D。要求启用 math 且 D > 0；
        /// D=0 在相应使用处编译失败。成功或失败均不修改源值或外部状态。
        ///
        /// # 错误
        /// 对数输入非正返回 Domain；指数函数接受所有输入。正确舍入后的 raw 越界
        /// 返回 Overflow（无符号类型允许微小负结果舍入到零）。预算内无法证明
        /// 1 raw 保证返回 PrecisionExhausted。后端错误、NaN、未获独立界证明的无穷
        /// 或无效上下界返回 ComputationFailed。
        pub fn $name(self) -> Result<Self, MathError> {
            self.math(Function::$kind, false)
        }
        #[doc = $description]
        /// 计算对应指数或对数，以区间界证明正确舍入结果，恰好半值时远离零。
        /// 保留计数类别与分母，数值为 raw / D。要求启用 math 且 D > 0；
        /// D=0 在相应使用处编译失败。成功或失败均不修改源值或外部状态。
        ///
        /// # 错误
        /// 对数输入非正返回 Domain；指数函数接受所有输入。正确舍入后的 raw 越界
        /// 返回 Overflow（无符号类型允许微小负结果舍入到零）。预算内无法证明唯一
        /// 舍入 raw 返回 PrecisionExhausted。后端错误、NaN、未获独立界证明的无穷
        /// 或无效上下界返回 ComputationFailed。
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
            unary!(
                exp,
                exp_strict,
                Exp,
                "Natural exponential, e raised to this value."
            );
            unary!(exp2, exp2_strict, Exp2, "Two raised to this value.");
            unary!(ln, ln_strict, Ln, "Natural logarithm.");
            unary!(log2, log2_strict, Log2, "Base-two logarithm.");
            unary!(log10, log10_strict, Log10, "Base-ten logarithm.");
            /// 计算底数为正且不等于一的对数，与正确舍入结果最多相差 1 raw。
            /// 保留计数类别和分母（数值 = raw / D）；要求启用 math 且 D > 0。
            /// 以半值远离零为目标舍入规则；成功或失败均不修改任一操作数或外部状态。
            ///
            /// # 错误
            /// 输入/底数非正或底数为一返回 Domain。正确舍入后的 raw 越界返回 Overflow；
            /// 无符号类型允许微小负结果舍入到零。有界精度搜索无法证明 1 raw 保证时
            /// 返回 PrecisionExhausted，包括对数分母区间仍含零的情况。后端错误、NaN、
            /// 未获独立界证明的无穷或无效区间返回 ComputationFailed。
            pub fn log(self, base: Self) -> Result<Self, MathError> {
                self.math(Function::Log(base.raw() as i128), false)
            }
            /// 计算底数为正且不等于一的正确舍入对数。
            /// 保留计数类别和分母（数值 = raw / D）；要求启用 math 且 D > 0。
            /// 恰好半值时远离零；成功或失败均不修改任一操作数或外部状态。
            ///
            /// # 错误
            /// 输入/底数非正或底数为一返回 Domain。正确舍入后的 raw 越界返回 Overflow；
            /// 无符号类型允许微小负结果舍入到零。有界精度搜索无法证明唯一舍入 raw 时
            /// 返回 PrecisionExhausted，包括对数分母区间仍含零的情况。后端错误、NaN、
            /// 未获独立界证明的无穷或无效区间返回 ComputationFailed。
            pub fn log_strict(self, base: Self) -> Result<Self, MathError> {
                self.math(Function::Log(base.raw() as i128), true)
            }
            /// 整数幂，负指数取倒数，定义 0^0 = 1。
            /// 与正确半值远离零舍入结果最多相差 1 raw。接受所有 i32 指数，包括 i32::MIN。
            /// 保留计数类别与分母（数值 = raw / D），要求启用 math 且 D > 0。
            /// 无论结果如何，源值与外部状态均保持不变。
            ///
            /// # 错误
            /// 零的负次幂返回 Domain。正确舍入后的 raw 越界返回 Overflow；无符号类型
            /// 允许负结果舍入到零。预算内无法证明 1 raw 保证返回 PrecisionExhausted，
            /// 包括取倒数前区间出现零端点。后端错误、NaN、未获独立界证明的无穷或无效区间
            /// 返回 ComputationFailed，包括取倒数之前正幂计算已失败的情况。
            pub fn powi(self, n: i32) -> Result<Self, MathError> {
                self.math(Function::Pow(n), false)
            }
            /// 正确舍入的整数幂，定义 0^0 = 1。
            /// 负指数取倒数；接受所有 i32 指数，包括 i32::MIN。保留计数类别与分母
            /// （数值 = raw / D），要求启用 math 且 D > 0，恰好半值时远离零。
            /// 无论结果如何，源值与外部状态均保持不变。
            ///
            /// # 错误
            /// 零的负次幂返回 Domain。正确舍入后的 raw 越界返回 Overflow；无符号类型
            /// 允许负结果舍入到零。预算内无法证明唯一舍入 raw 返回 PrecisionExhausted，
            /// 包括取倒数前区间出现零端点。后端错误、NaN、未获独立界证明的无穷或无效区间
            /// 返回 ComputationFailed，包括取倒数之前正幂计算已失败的情况。
            pub fn powi_strict(self, n: i32) -> Result<Self, MathError> {
                self.math(Function::Pow(n), true)
            }
        }
    };
}
math_methods!(FixedI64, i64);
math_methods!(FixedU64, u64);

// 快速档独立于区间求值和自适应 compute；所有后端请求精度固定在此处。
// 单值链路采用 ToEven，既不包围数学真值，也不证明 1 raw 误差界；
// 最后的 quantize 才按半值远离零转换成 raw，不能将两种舍入规则混为一谈。
const FAST_PRECISION: usize = 128;

fn fast_checked(value: BigFloat) -> Result<BigFloat, MathError> {
    validate_endpoint(&value)?;
    Ok(value)
}

fn fast_ratio(raw: i128, d: u32) -> Result<BigFloat, MathError> {
    let p = FAST_PRECISION;
    let rounding = R::ToEven;
    fast_checked(fast_request!(
        "ratio.div",
        p,
        rounding,
        integer(raw, p).div(&BigFloat::from_u32(d, p), p, rounding)
    ))
}

fn fast_evaluate(
    raw: i128,
    d: u32,
    function: Function,
    cc: &mut Consts,
) -> Result<BigFloat, MathError> {
    let p = FAST_PRECISION;
    let rounding = R::ToEven;
    let x = fast_ratio(raw, d)?;
    let result = match function {
        Function::Exp => fast_checked(fast_request!("exp", p, rounding, x.exp(p, rounding, cc)))?,
        Function::Exp2 => {
            let two = integer(2, p);
            if raw % i128::from(d) == 0 {
                // 调用方已将指数限制在 [-100, 100]；整数幂避免通用 pow
                // 在精确二进制结果附近进行不必要的内部舍入确认。
                let n = raw / i128::from(d);
                let power = fast_checked(fast_request!(
                    "exp2.powi",
                    p,
                    rounding,
                    two.powi(n.unsigned_abs() as usize, p, rounding)
                ))?;
                if n < 0 {
                    if power.is_zero() {
                        return Err(MathError::ComputationFailed);
                    }
                    fast_checked(fast_request!(
                        "power.reciprocal",
                        p,
                        rounding,
                        power.reciprocal(p, rounding)
                    ))?
                } else {
                    power
                }
            } else {
                fast_checked(fast_request!(
                    "exp2.pow",
                    p,
                    rounding,
                    two.pow(&x, p, rounding, cc)
                ))?
            }
        }
        Function::Ln => fast_checked(fast_request!("ln", p, rounding, x.ln(p, rounding, cc)))?,
        Function::Log2 => {
            fast_checked(fast_request!("log2", p, rounding, x.log2(p, rounding, cc)))?
        }
        Function::Log10 => fast_checked(fast_request!(
            "log10",
            p,
            rounding,
            x.log10(p, rounding, cc)
        ))?,
        Function::Log(base) => {
            let b = fast_ratio(base, d)?;
            let numerator = fast_checked(fast_request!("ln", p, rounding, x.ln(p, rounding, cc)))?;
            let denominator = fast_checked(fast_request!(
                "log.base.ln",
                p,
                rounding,
                b.ln(p, rounding, cc)
            ))?;
            if denominator.is_zero() {
                return Err(MathError::ComputationFailed);
            }
            fast_checked(fast_request!(
                "log.div",
                p,
                rounding,
                numerator.div(&denominator, p, rounding)
            ))?
        }
        Function::Pow(n) => {
            // raw 来自 64 位计数，提升到 i128 后 abs 安全；unsigned_abs
            // 可处理 i32::MIN，在支持的 32/64 位平台上均可表示为 usize。
            let magnitude = fast_ratio(raw.abs(), d)?;
            let mut power = fast_checked(fast_request!(
                "powi",
                p,
                rounding,
                magnitude.powi(n.unsigned_abs() as usize, p, rounding)
            ))?;
            if n < 0 {
                // 必须在倒数前检查：无穷不能变成伪精确零，零也不能倒数。
                if power.is_zero() {
                    return Err(MathError::ComputationFailed);
                }
                power = fast_checked(fast_request!(
                    "power.reciprocal",
                    p,
                    rounding,
                    power.reciprocal(p, rounding)
                ))?;
            }
            if raw < 0 && n % 2 != 0 {
                power = fast_checked(power.neg())?;
            }
            power
        }
    };
    fast_checked(fast_request!(
        "scale.mul",
        p,
        rounding,
        result.mul(&BigFloat::from_u32(d, p), p, rounding)
    ))
}

fn fast_compute(
    raw: i128,
    d: u32,
    function: Function,
    min: i128,
    max: i128,
) -> Result<i128, MathError> {
    match function {
        Function::Ln | Function::Log2 | Function::Log10 if raw <= 0 => {
            return Err(MathError::Domain);
        }
        Function::Log(base) if raw <= 0 || base <= 0 || base == i128::from(d) => {
            return Err(MathError::Domain);
        }
        Function::Pow(n) if raw == 0 && n < 0 => return Err(MathError::Domain),
        // 零次幂已经是 raw 单位，不再经过统一乘 D；包括 0^0。
        Function::Pow(0) => return Ok(i128::from(d)),
        // 2^100 > u64::MAX，e^100 > 2^100；反向时两者乘
        // u32::MAX 仍小于半个 raw。这是独立数学界，不依赖近似误差。
        Function::Exp | Function::Exp2 if raw > 100 * i128::from(d) => {
            return Err(MathError::Overflow);
        }
        Function::Exp | Function::Exp2 if raw < -100 * i128::from(d) => return Ok(0),
        _ => {}
    }
    let mut cc = Consts::new().map_err(|_| MathError::ComputationFailed)?;
    let value = fast_evaluate(raw, d, function, &mut cc)?;
    // 复用精确二进制半值比较，不复用任何上下界或升精度控制流。
    let rounded = quantize(&value, FAST_PRECISION)?;
    if rounded < min || rounded > max {
        Err(MathError::Overflow)
    } else {
        Ok(rounded)
    }
}

macro_rules! fast_unary {
    ($name:ident, $kind:ident, $description:literal) => {
        #[doc = $description]
        /// 固定请求精度的指数或对数近似：exp/exp2 分别为自然/二次指数，ln/log2/log10 为对应对数。
        /// 要求启用 math 且 D > 0，D=0 在相应使用处编译失败。
        /// 保留计数类别与分母（数值 = raw / D）。每次后端算术/函数请求均为 128 bit、
        /// ToEven（最近且半值取偶）舍入，再以相同精度乘 D，最后将近似 raw 按半值远离零量化。
        /// 不构建上下界、不自适应升精度、不回退，也不提供统一 1 raw 数学误差保证。
        /// 后端保护位不受限制；不保证固定耗时或加速比例。在舍入或范围边界附近
        /// 可能与普通/严格方法不同。成功或失败均不修改输入或外部状态。
        ///
        /// # 错误
        /// 对数输入非正返回 Domain；指数函数接受所有输入。量化后的近似 raw 越界，
        /// 或独立数学界证明越界时返回 Overflow；无符号类型允许负近似量化为零。
        /// 后端错误、NaN 或无独立界支持的无穷返回 ComputationFailed。
        /// 绝不返回 PrecisionExhausted。
        pub fn $name(self) -> Result<Self, MathError> {
            self.fast_math(Function::$kind)
        }
    };
}

macro_rules! fast_math_methods {
    ($name:ident, $ty:ty) => {
        impl<const D: u32> $name<D> {
            fn fast_math(self, function: Function) -> Result<Self, MathError> {
                const { assert!(D > 0, "denominator must be positive") };
                fast_compute(
                    self.raw() as i128,
                    D,
                    function,
                    <$ty>::MIN as i128,
                    <$ty>::MAX as i128,
                )
                .map(|raw| Self::from_raw(raw as $ty))
            }

            fast_unary!(
                fast_exp,
                Exp,
                "Fixed-budget approximation of e raised to this value."
            );
            fast_unary!(
                fast_exp2,
                Exp2,
                "Fixed-budget approximation of two raised to this value."
            );
            fast_unary!(
                fast_ln,
                Ln,
                "Fixed-budget approximation of the natural logarithm."
            );
            fast_unary!(
                fast_log2,
                Log2,
                "Fixed-budget approximation of the base-two logarithm."
            );
            fast_unary!(
                fast_log10,
                Log10,
                "Fixed-budget approximation of the base-ten logarithm."
            );

            /// 固定请求精度的对数近似，计算 ln(self) / ln(base)。
            /// 两操作数类型和分母相同，结果保留计数类别与分母（数值 = raw / D）。
            /// 要求启用 math 且 D > 0，D=0 在相应使用处编译失败。每次后端算术/函数请求
            /// 均为 128 bit、ToEven（最近且半值取偶），包括最终乘 D；近似 raw 按半值远离零量化。
            /// 不构建上下界、不升精度、不回退，也不提供统一 1 raw 误差保证。后端保护位
            /// 不受限制，不保证固定耗时或加速比例。舍入/范围边界附近可能与普通/严格方法不同。
            /// 无论结果如何，任一操作数与外部状态均保持不变。
            ///
            /// # 错误
            /// 输入/底数非正或底数为一返回 Domain。量化近似 raw 越界返回 Overflow；
            /// 无符号类型允许负近似量化为零。后端错误、NaN、未获独立界证明的无穷，
            /// 或近似 ln(base) 为零（即使精确底数合法）返回 ComputationFailed。
            /// 绝不返回 PrecisionExhausted。
            pub fn fast_log(self, base: Self) -> Result<Self, MathError> {
                self.fast_math(Function::Log(base.raw() as i128))
            }

            /// 固定请求精度的整数幂近似，接受所有 i32 指数，包括 i32::MIN。
            /// 负指数取倒数；负底数按指数奇偶保留符号。定义 0^0 = 1，直接返回 D raw。
            /// 要求启用 math 且 D > 0，D=0 在相应使用处编译失败。结果保留计数类别与分母
            /// （数值 = raw / D）。每次后端算术/函数请求均为 128 bit、ToEven（最近且半值取偶）；
            /// 最终乘 D 也为 128 bit，然后将近似 raw 按半值远离零量化。不构建上下界、
            /// 不升精度、不回退，也不提供统一 1 raw 误差保证。后端保护位不受限制，
            /// 不保证固定耗时或加速比例。舍入/范围边界附近可能与普通/严格方法不同。
            /// 成功或失败均不修改输入或外部状态。
            ///
            /// # 错误
            /// 零的负次幂返回 Domain。量化近似 raw 越界返回 Overflow；无符号类型允许
            /// 负近似量化为零。后端错误、NaN、未获独立界证明的无穷或取倒数前无效的
            /// 中间幂（包括零）返回 ComputationFailed；不会把无穷的正幂静默取倒数为零。
            /// 绝不返回 PrecisionExhausted。
            pub fn fast_powi(self, n: i32) -> Result<Self, MathError> {
                self.fast_math(Function::Pow(n))
            }
        }
    };
}

fast_math_methods!(FixedI64, i64);
fast_math_methods!(FixedU64, u64);

#[cfg(test)]
#[path = "math_inner_test.rs"]
mod math_inner_test;
