#[path = "support/decimal_api_inner.rs"]
mod decimal_api_inner;

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: decimal_strings_round_without_floating_point
 *   Given D10 文本 -1.25/-0.05；D100 文本 1.234/1.235/1.25e2/0.001e1/0.005e1/-1.25e-2；最大分母的 1；D1 的 -9223372036854775808/18446744073709551615
 *   When 解析到相应有符号或无符号数值
 *   Then raw 依次为 -13/-1、123/124/12500/1/5/-1、4294967295、i64::MIN/u64::MAX
 */
#[test]
fn decimal_strings_round_without_floating_point() {
    decimal_api_inner::decimal_strings_round_without_floating_point();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: invalid_or_out_of_range_text_is_rejected
 *   Given D10 的非法文本 nope、无符号文本 -1/18446744073709551616
 *   When 解析文本
 *   Then nope 返回 Invalid，其余返回 Overflow
 */
#[test]
fn invalid_or_out_of_range_text_is_rejected() {
    decimal_api_inner::invalid_or_out_of_range_text_is_rejected();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md「十进制输入、输出与序列化」；切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario: unsigned_negative_text_rounds_to_zero
 *   Given 无符号目标 D10 及文本 -0.01
 *   When 解析文本并按半值远离零舍入
 *   Then 缩放后的 -0.1 舍入为 raw 0，成功返回，源文本不变
 */
#[test]
fn unsigned_negative_text_rounds_to_zero() {
    decimal_api_inner::unsigned_negative_text_rounds_to_zero();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: decimal_display_shows_expected_values
 *   Given 无符号 raw 123/D100、3/D3，以及有符号 -5/D10、0/D10
 *   When 输出十进制文本
 *   Then 分别显示 1.23、1、-0.5、0
 */
#[test]
fn decimal_display_shows_expected_values() {
    decimal_api_inner::decimal_display_shows_expected_values();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md「十进制输入、输出与序列化」的十进制可读输出；切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario Outline: repeating_decimal_display_has_readable_decimal_format_only
 *   Given 无符号 raw 1/D3 或 2/D3
 *   When 生成 Display 文本
 *   Then 文本符合十进制数字格式，能解析为有限数值，源 raw 不变
 *   Examples: raw 1；raw 2
 *   本场景仅验收格式，不验收对应数值或精度；契约未规定循环小数输出误差，故不宣称完成其数值验收。
 */
#[test]
fn repeating_decimal_display_has_readable_decimal_format_only() {
    decimal_api_inner::repeating_decimal_display_has_readable_decimal_format_only();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: debug_includes_raw_and_denominator
 *   Given 无符号 raw 123/D100 及有符号 raw -12/D10
 *   When 生成 Debug 文本
 *   Then 分别包含数值 123 与 100、-12 与 10，源 raw 不变；不限定标签或排版
 */
#[test]
fn debug_includes_raw_and_denominator() {
    decimal_api_inner::debug_includes_raw_and_denominator();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: numeric_input_rounds_decimal_half_ties
 *   Given D10 有符号 -1.25f64 及无符号 1.25f32
 *   When 经直接浮点或 TryFrom 入口转换
 *   Then 分别得到 raw -13、13
 */
#[test]
fn numeric_input_rounds_decimal_half_ties() {
    decimal_api_inner::numeric_input_rounds_decimal_half_ties();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: nonfinite_numeric_input_is_rejected
 *   Given D10 无符号目标的 f64 NaN 与有符号目标的 f64 正无穷
 *   When 经直接浮点入口转换
 *   Then 均返回 Overflow
 */
#[test]
fn nonfinite_numeric_input_is_rejected() {
    decimal_api_inner::nonfinite_numeric_input_is_rejected();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: primitive_integer_input_scales_or_reports_overflow
 *   Given D10 的整数 100u8/100u64、无符号目标的 u128::MAX/-1i8
 *   When 经泛型整数或 TryFrom 入口转换
 *   Then 100 缩放为 raw 1000，u128::MAX 与 -1 返回 Overflow
 */
#[test]
fn primitive_integer_input_scales_or_reports_overflow() {
    decimal_api_inner::primitive_integer_input_scales_or_reports_overflow();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md；design/pi-fixed-scale-int/07-design/call-chain.md 切片 7（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: finite_numeric_input_checks_final_range
 *   Given D1 无符号目标的 f64 1e-100 与 1e100
 *   When 经直接浮点入口转换
 *   Then 微小正值得 raw 0，超大正值返回 Overflow
 */
#[test]
fn finite_numeric_input_checks_final_range() {
    decimal_api_inner::finite_numeric_input_checks_final_range();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md；design/pi-fixed-scale-int/07-design/call-chain.md 切片 8（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * Scenario: mixed_operands_keep_left_type_and_scale
 *   Given 左值 raw 150/D100，右值 raw 5/D10 或整数 100/2/-2/0；无符号 raw 2/D10 与有符号 -1/D2；D10 的 -1 与 4 或 1/D2；u64::MAX 与 2；i64::MIN 与 u64::MAX；u64::MAX 与 -1
 *   When 调用普通、舍入或饱和的混合乘除
 *   Then 结果沿左值类型和分母，150×(5/10) 得 raw 75，150÷(5/10) 得 raw 300，整数乘100/除2/乘-2 得15000/75/-300；负数不可表示和超界报 Overflow、除零报 DivisionByZero；舍入得0/-1，饱和夹u64::MAX或i64::MIN
 */
#[test]
fn mixed_operands_keep_left_type_and_scale() {
    decimal_api_inner::mixed_operands_keep_left_type_and_scale();
}
