// @design-doc:design/pi-fixed-scale-int/00-index.md
// @test:tests/ticket_7_test.rs
#[path = "support/ticket_7_test_inner.rs"]
mod ticket_7_test_inner;

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: text_rounding
 *   Given D10 文本 ±1.24、±1.25、±1.26、±0.05
 *   When FromStr 输入
 *   Then raw 为 ±12、±13、±13、±1，源文本不变
 */
#[test]
fn text_rounding() {
    ticket_7_test_inner::text_rounding();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: text_exponents_and_denominator
 *   Given D100 的 +1.25e2、-1.25E-2；最大分母下文本 1
 *   When 解析文本
 *   Then raw 分别为 12500、-1、u32::MAX
 */
#[test]
fn text_exponents_and_denominator() {
    ticket_7_test_inner::text_exponents_and_denominator();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: invalid_text
 *   Given 空串、仅符号、小数点、重复点/指数、缺指数、空白、非数字、非有限文本
 *   When 输入两类 FromStr
 *   Then 均返回 Invalid 且源文本不变
 */
#[test]
fn invalid_text() {
    ticket_7_test_inner::invalid_text();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: text_rounded_bounds
 *   Given D1 的 i64/u64 极值加不足半值或半值，以及 D10 的 -0.049/-0.05
 *   When 解析并舍入
 *   Then 不足半值保留极值，越界半值报 Overflow；无符号微负舍入为零或报错
 */
#[test]
fn text_rounded_bounds() {
    ticket_7_test_inner::text_rounded_bounds();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: long_text
 *   Given 10001 个前导零后接 1、1.25 后接 10001 个零、5 后接 10001 个零再接 e-10003
 *   When 按 D10 输入
 *   Then 分别得 raw 10、13、1，不施加未约定的数字长度上限
 */
#[test]
fn long_text() {
    ticket_7_test_inner::long_text();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: float_ties_zero_subnormal
 *   Given f32/f64 的 ±1.25、±0、最小正负次正规值
 *   When 经显式及 TryFrom 入口转 D10
 *   Then raw 分别 ±13、0、0，源位模式不变
 */
#[test]
fn float_ties_zero_subnormal() {
    ticket_7_test_inner::float_ties_zero_subnormal();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: f64_decimal_rounding
 *   Given f64 ±0.15 的最短往返十进制文本为 ±0.15
 *   When 经两入口转 D10
 *   Then raw 为 2 和 -2，按十进制半值远离零，源位模式不变
 */
#[test]
fn f64_decimal_rounding() {
    ticket_7_test_inner::f64_decimal_rounding();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: f32_decimal_rounding
 *   Given f32 ±0.35 的原始类型最短往返十进制文本为 ±0.35
 *   When 经两入口转 D10
 *   Then raw 为 4 和 -4，不先提升为 f64，源位模式不变
 */
#[test]
fn f32_decimal_rounding() {
    ticket_7_test_inner::f32_decimal_rounding();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: large_float_decimal_value
 *   Given f64 的 ±2^60，最短往返十进制文本为 ±1152921504606847000
 *   When 转 D1
 *   Then raw 为 ±1152921504606847000，而非二进制精确整数，源位模式不变
 */
#[test]
fn large_float_decimal_value() {
    ticket_7_test_inner::large_float_decimal_value();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: nonfinite_float
 *   Given f32/f64 NaN 和正负无穷
 *   When 经显式及 TryFrom 入口输入两类值
 *   Then 均返回 Overflow 且源位模式不变
 */
#[test]
fn nonfinite_float() {
    ticket_7_test_inner::nonfinite_float();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: float_range
 *   Given D1 的 f64 ±2^63 文本为 ±9223372036854776000，2^64 文本为 18446744073709552000；D10 的无符号目标负值和 f32::MAX
 *   When 转换到对应目标数值
 *   Then 所有输入均因十进制缩放后的 raw 越界报 Overflow，源位模式不变
 */
#[test]
fn float_range() {
    ticket_7_test_inner::float_range();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: integer_inputs
 *   Given D100 的整数 2/-2、D1 的两类极值、D2 极值、u128::MAX、i128::MAX、无符号目标的 -1
 *   When 经完整整数、泛型整数及 TryFrom 入口构造
 *   Then 缩放得 ±200，D1 极值保持，超界报 Overflow 且源值不变
 */
#[test]
fn integer_inputs() {
    ticket_7_test_inner::integer_inputs();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: decimal_display
 *   Given raw 123/D100、-5/D10、零及 D1 的 i64::MIN/u64::MAX
 *   When 调用 Display
 *   Then 显示 1.23、-0.5、0 和完整极值十进制，源 raw 不变
 */
#[test]
fn decimal_display() {
    ticket_7_test_inner::decimal_display();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md:47–49；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: debug_raw_denominator
 *   Given 无符号 raw 123/D100 及有符号 raw -13/D10
 *   When 调用 Debug
 *   Then 同时显示 raw 和分母及其值，源值不变
 */
#[test]
fn debug_raw_denominator() {
    ticket_7_test_inner::debug_raw_denominator();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md「十进制输入、输出与序列化」浮点验收；src/lib.rs try_from_f32/try_from_f64（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @critical
 * Scenario Outline: float_shortest_decimal_at_billion_scale
 *   Given 原始 f32/f64 及文本的 ±0.1，分母 1000000000
 *   When 经直接浮点、TryFrom 或文本入口转换
 *   Then 有符号 raw 分别为 ±100000000，无符号正值为 100000000，源浮点位模式不变
 *   Examples: f32 0.1/-0.1；f64 0.1/-0.1；文本 "0.1"/"-0.1"
 */
#[test]
fn float_shortest_decimal_at_billion_scale() {
    ticket_7_test_inner::float_shortest_decimal_at_billion_scale();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md「十进制输入、输出与序列化」；src/lib.rs try_from_f32/try_from_f64（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: unsigned_negative_float_rounding_boundary
 *   Given D10 无符号目标，原始 f32/f64 的 -0.049、-0.05、-0.051
 *   When 经直接浮点或 TryFrom 入口转换
 *   Then -0.049 舍入为 raw 0；-0.05 和 -0.051 舍入后仍负，返回 Overflow；源位模式不变
 *   Examples: f32 与 f64，直接入口与 TryFrom 各自覆盖三个数值
 */
#[test]
fn unsigned_negative_float_rounding_boundary() {
    ticket_7_test_inner::unsigned_negative_float_rounding_boundary();
}
