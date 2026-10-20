#[path = "support/ticket_3_test_inner.rs"]
mod ticket_3_test_inner;

/** 契约：src/lib.rs:54-59,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 同分母原始计数相加得到精确和
 *   Given 分母 100 的无符号 150、50，或有符号 -150、50
 *   When 调用 checked_add
 *   Then 分别读回 200 (2.00) 或 -100 (-1.00)，两操作数仍为原值
 */
#[test]
fn checked_add_returns_exact_raw_sum() {
    ticket_3_test_inner::checked_add_returns_exact_raw_sum();
}

/** 契约：src/lib.rs:61-66,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 同分母原始计数相减得到精确差
 *   Given 分母 100 的无符号 150、50，或有符号 -150、50，或有符号 150、-50
 *   When 调用 checked_sub
 *   Then 分别读回 100 (1.00)、-200 (-2.00) 或 200 (2.00)，两操作数仍为原值
 */
#[test]
fn checked_sub_returns_exact_raw_difference() {
    ticket_3_test_inner::checked_sub_returns_exact_raw_difference();
}

/** 契约：src/lib.rs:54-59,17-21,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 正向加法超出类别最大计数时报 Overflow
 *   Given 分母 10 的 u64::MAX 或 i64::MAX，及原始计数 1
 *   When 调用 checked_add
 *   Then 均返回 Overflow，两个原始操作数不变
 */
#[test]
fn checked_add_above_max_overflows() {
    ticket_3_test_inner::checked_add_above_max_overflows();
}

/** 契约：src/lib.rs:54-59,17-21,185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario: 有符号负向加法超出最小计数时报 Overflow
 *   Given 分母 10 的原始计数 i64::MIN 和 -1
 *   When 调用 checked_add
 *   Then 返回 Overflow，两个原始操作数不变
 */
#[test]
fn checked_add_below_signed_min_overflows() {
    ticket_3_test_inner::checked_add_below_signed_min_overflows();
}

/** 契约：src/lib.rs:61-66,17-21,184
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 无符号左计数小于右计数时报 Overflow
 *   Given 分母 10 的无符号原始计数 0 和 1，或 2 和 5
 *   When 调用 checked_sub
 *   Then 均返回 Overflow，两组原始操作数不变
 */
#[test]
fn checked_unsigned_sub_below_zero_overflows() {
    ticket_3_test_inner::checked_unsigned_sub_below_zero_overflows();
}

/** 契约：src/lib.rs:61-66,17-21,185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 有符号减法越过最大或最小计数时报 Overflow
 *   Given 分母 10 的 i64::MIN 减 1，或 i64::MAX 减 -1
 *   When 调用 checked_sub
 *   Then 两次均返回 Overflow，两组操作数不变
 */
#[test]
fn checked_signed_sub_outside_bounds_overflows() {
    ticket_3_test_inner::checked_signed_sub_outside_bounds_overflows();
}

/** 契约：src/lib.rs:83-88,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 未超界的饱和加法不改变精确结果
 *   Given 分母 100 的无符号 150、50，或有符号 -150、50
 *   When 调用 saturating_add
 *   Then 分别读回 200 或 -100，两个原始操作数不变
 */
#[test]
fn saturating_add_without_overflow_returns_exact_sum() {
    ticket_3_test_inner::saturating_add_without_overflow_returns_exact_sum();
}

/** 契约：src/lib.rs:90-95,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 未超界的饱和减法不改变精确结果
 *   Given 分母 100 的无符号 150、50，或有符号 -150、50，或有符号 150、-50
 *   When 调用 saturating_sub
 *   Then 分别读回 100、-200 或 200，各组操作数不变
 */
#[test]
fn saturating_sub_without_overflow_returns_exact_difference() {
    ticket_3_test_inner::saturating_sub_without_overflow_returns_exact_difference();
}

/** 契约：src/lib.rs:83-88,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 正向饱和加法夹至类别最大计数
 *   Given 分母 10 的 u64::MAX 或 i64::MAX，及原始计数 1
 *   When 调用 saturating_add
 *   Then 分别读回 u64::MAX 或 i64::MAX，两个原始操作数不变
 */
#[test]
fn saturating_add_above_max_clamps_to_max() {
    ticket_3_test_inner::saturating_add_above_max_clamps_to_max();
}

/** 契约：src/lib.rs:83-88,185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario: 有符号负向饱和加法夹至最小计数
 *   Given 分母 10 的原始计数 i64::MIN 和 -1
 *   When 调用 saturating_add
 *   Then 读回 i64::MIN，两个原始操作数不变
 */
#[test]
fn saturating_add_below_signed_min_clamps_to_min() {
    ticket_3_test_inner::saturating_add_below_signed_min_clamps_to_min();
}

/** 契约：src/lib.rs:90-95,184
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 无符号左计数小于右计数时饱和为零
 *   Given 分母 10 的无符号原始计数 0 和 1，或 2 和 5
 *   When 调用 saturating_sub
 *   Then 均读回 0，两组原始操作数不变
 */
#[test]
fn saturating_unsigned_sub_below_zero_clamps_to_zero() {
    ticket_3_test_inner::saturating_unsigned_sub_below_zero_clamps_to_zero();
}

/** 契约：src/lib.rs:90-95,185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 有符号饱和减法夹至最小或最大计数
 *   Given 分母 10 的 i64::MIN 减 1，或 i64::MAX 减 -1
 *   When 调用 saturating_sub
 *   Then 分别读回 i64::MIN 或 i64::MAX，两组操作数不变
 */
#[test]
fn saturating_signed_sub_outside_bounds_clamps() {
    ticket_3_test_inner::saturating_signed_sub_outside_bounds_clamps();
}
