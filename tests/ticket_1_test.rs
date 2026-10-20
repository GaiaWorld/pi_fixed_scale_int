#[path = "support/ticket_1_test_inner.rs"]
mod ticket_1_test_inner;

/** 契约：src/lib.rs:25-45
 * @critical
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario: 123 个百分之一单位表示 1.23
 *   Given 分母 100 和原始计数 123
 *   When 构造并读取数值
 *   Then 读回 123，即 1.23
 */
#[test]
fn raw_123_at_scale_100_represents_1_23() {
    ticket_1_test_inner::raw_123_at_scale_100_represents_1_23();
}

/** 契约：src/lib.rs:7-15,25-45
 * @critical
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario Outline: 九个便捷分母与同值自定义分母一致
 *   Given 原始计数 123 与九个便捷分母
 *   When 分别构造并读取便捷与同值自定义分母的数值
 *   Then 每对数值具有相同的分母与原始计数
 *   Examples: 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1371600
 */
#[test]
fn convenient_scales_match_custom_scales() {
    ticket_1_test_inner::convenient_scales_match_custom_scales();
}

/** 契约：src/lib.rs:25-45
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario: 最大 u32 分母仍可使用
 *   Given 分母 u32::MAX 和原始计数 123 与 -123
 *   When 构造并读取两类数值
 *   Then 分别读回 123 和 -123
 */
#[test]
fn max_u32_denominator_is_usable() {
    ticket_1_test_inner::max_u32_denominator_is_usable();
}

/** 契约：src/lib.rs:25-45,184
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario Outline: 无符号原始计数边界可读回
 *   Given 分母 100 与计数 0 或 u64::MAX
 *   When 分别构造并读取
 *   Then 每个值读回其原始计数
 *   Examples: 0, u64::MAX
 */
#[test]
fn unsigned_raw_count_boundaries_round_trip() {
    ticket_1_test_inner::unsigned_raw_count_boundaries_round_trip();
}

/** 契约：src/lib.rs:25-45,185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario Outline: 带符号原始计数边界可读回
 *   Given 分母 100 与计数 i64::MIN 或 i64::MAX
 *   When 分别构造并读取
 *   Then 每个值读回其原始计数
 *   Examples: i64::MIN, i64::MAX
 */
#[test]
fn signed_raw_count_boundaries_round_trip() {
    ticket_1_test_inner::signed_raw_count_boundaries_round_trip();
}

/** 契约：src/lib.rs:17-21
 * @critical
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario: 两个公开算术错误变体可区分
 *   Given 错误类型 ArithmeticError
 *   When 分别取得 Overflow 与 DivisionByZero
 *   Then 两个公开错误值均可使用，且彼此不同
 */
#[test]
fn arithmetic_error_variants_are_public_and_distinct() {
    ticket_1_test_inner::arithmetic_error_variants_are_public_and_distinct();
}

/** 契约：src/lib.rs:25-45,128-141,159-179
 * @negative
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario Outline: 零分母用于公开操作时编译失败
 *   Given 分母零的公开构造、读取、比较或目标转换程序，读取与比较的值不通过公开构造生成
 *   When 分别编译程序
 *   Then 每个程序均因零分母约束编译失败
 *   Examples: 构造、读取、比较、目标转换
 */
#[test]
fn zero_denominator_public_operations_fail_to_compile() {
    ticket_1_test_inner::zero_denominator_public_operations_fail_to_compile();
}
