#[path = "support/ticket_2_test_inner.rs"]
mod ticket_2_test_inner;

/** 契约：src/lib.rs:25-52,184
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario: 无符号原始计数 2 与完整整数 2 不混淆
 *   Given 无符号分母 100 与输入 2
 *   When 分别从原始计数和完整整数构造并读取
 *   Then 原始计数分别为 2 和 200，即 0.02 和 2.00
 */
#[test]
fn unsigned_raw_and_whole_two_have_distinct_counts() {
    ticket_2_test_inner::unsigned_raw_and_whole_two_have_distinct_counts();
}

/** 契约：src/lib.rs:25-52,185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 有符号完整整数放大后可读取原始计数
 *   Given 有符号分母 100 与完整整数 -2、0 或 2
 *   When 从完整整数构造并读取
 *   Then 分别读回 -200、0 或 200
 *   Examples: -2 -> -200, 0 -> 0, 2 -> 200
 */
#[test]
fn signed_whole_integers_scale_with_sign() {
    ticket_2_test_inner::signed_whole_integers_scale_with_sign();
}

/** 契约：src/lib.rs:47-52,184
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario: 无符号完整整数恰好达到可表示的放大上界
 *   Given 分母 100 与完整整数 u64::MAX / 100
 *   When 从完整整数构造并读取
 *   Then 读回 (u64::MAX / 100) * 100
 */
#[test]
fn unsigned_whole_integer_at_scaling_limit_succeeds() {
    ticket_2_test_inner::unsigned_whole_integer_at_scaling_limit_succeeds();
}

/** 契约：src/lib.rs:17-21,47-52,184
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario: 无符号完整整数放大超界返回 Overflow
 *   Given 分母 100 与完整整数 u64::MAX / 100 + 1
 *   When 从完整整数构造
 *   Then 返回 Overflow 而不是饱和计数
 */
#[test]
fn unsigned_whole_integer_above_scaling_limit_overflows() {
    ticket_2_test_inner::unsigned_whole_integer_above_scaling_limit_overflows();
}

/** 契约：src/lib.rs:47-52,185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 有符号完整整数在正负放大边界可表示
 *   Given 分母 100 与完整整数 i64::MAX / 100 或 i64::MIN / 100
 *   When 分别从完整整数构造并读取
 *   Then 分别读回相应整数乘以 100 的原始计数
 *   Examples: i64::MAX / 100, i64::MIN / 100
 */
#[test]
fn signed_whole_integers_at_scaling_limits_succeed() {
    ticket_2_test_inner::signed_whole_integers_at_scaling_limits_succeed();
}

/** 契约：src/lib.rs:17-21,47-52,185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 有符号完整整数超出正负放大边界均返回 Overflow
 *   Given 分母 100 与完整整数 i64::MAX / 100 + 1 或 i64::MIN / 100 - 1
 *   When 分别从完整整数构造
 *   Then 两次均返回 Overflow 而非饱和计数
 *   Examples: 正向超界、负向超界
 */
#[test]
fn signed_whole_integers_outside_scaling_limits_overflow() {
    ticket_2_test_inner::signed_whole_integers_outside_scaling_limits_overflow();
}

/** 契约：src/lib.rs:159-179,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario Outline: 同分母计数可以相等比较和排序
 *   Given 无符号分母 100 的计数 2、2、200 与有符号分母 100 的计数 -200、-2、-2、0、2
 *   When 分别比较并排序同类别同分母数值
 *   Then 两类相同计数均相等、不同计数均不等，且排序分别为 2、2、200 与 -200、-2、0、2
 *   Examples: 无符号、有符号
 */
#[test]
fn same_scale_values_compare_and_sort_by_count() {
    ticket_2_test_inner::same_scale_values_compare_and_sort_by_count();
}

/** 契约：src/lib.rs:25-30,54-58,159-179
 * @negative
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario Outline: 不同分母不能直接混合运算或比较
 *   Given 同类别的分母 10 与 100 的原始计数值
 *   When 将不同分母的值直接用于同分母加法或比较
 *   Then 程序因分母类型不匹配而无法编译
 *   Examples: 无符号加法、有符号加法、无符号比较、有符号比较
 */
#[test]
fn different_scales_cannot_be_mixed_directly() {
    ticket_2_test_inner::different_scales_cannot_be_mixed_directly();
}

/** 契约：src/lib.rs:25-26,47-52,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * Scenario Outline: 零分母不能用于完整整数输入
 *   Given 无符号或有符号的分母零与完整整数 2
 *   When 调用公开的 try_from_integer 输入完整整数
 *   Then 两种调用均因零分母约束无法编译
 *   Examples: FixedU64<0>、FixedI64<0>
 */
#[test]
fn zero_denominator_whole_integer_input_fails_to_compile() {
    ticket_2_test_inner::zero_denominator_whole_integer_input_fails_to_compile();
}
