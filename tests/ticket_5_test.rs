#[path = "support/ticket_5_test_inner.rs"]
mod ticket_5_test_inner;

/** 契约：src/lib.rs:68-73,112-118,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 乘法半值远离零而默认向零截断
 *   Given D=10 的原始计数 1 与 5（u64/i64），或 -1 与 5（i64）
 *   When 分别调用 checked_mul 与 rounded_mul
 *   Then 截断结果均为 0，舍入结果分别为 1 或 -1，操作数不变
 */
#[test]
fn rounded_mul_half_ties() { ticket_5_test_inner::rounded_mul_half_ties(); }

/** 契约：src/lib.rs:75-81,120-126,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 除法半值远离零而默认向零截断
 *   Given D=10 的原始计数 1 与 4（u64/i64），或 -1 与 4（i64）
 *   When 分别调用 checked_div 与 rounded_div
 *   Then 截断结果分别为 2 或 -2，舍入结果分别为 3 或 -3，操作数不变
 */
#[test]
fn rounded_div_half_ties() { ticket_5_test_inner::rounded_div_half_ties(); }

/** 契约：src/lib.rs:68-73,112-118,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 乘法按半值阈值决定是否进位
 *   Given D=10 的原始计数 1 或 -1（i64）与 4 或 6，以及 1（u64）与 4 或 6
 *   When 分别调用 checked_mul 与 rounded_mul
 *   Then 1×4 与 -1×4 均截断为 0 且舍入为 0；1×6 与 -1×6 均截断为 0 且舍入为 1 或 -1，操作数不变
 */
#[test]
fn rounded_mul_either_side_of_half() { ticket_5_test_inner::rounded_mul_either_side_of_half(); }

/** 契约：src/lib.rs:75-81,120-126,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 除法按半值阈值决定是否进位
 *   Given D=10 的原始计数 1 或 -1（i64）与 7 或 6，以及 1（u64）与 7 或 6
 *   When 分别调用 checked_div 与 rounded_div
 *   Then 1÷7 与 -1÷7 截断及舍入均为 1 或 -1；1÷6 与 -1÷6 截断为 1 或 -1、舍入为 2 或 -2，操作数不变
 */
#[test]
fn rounded_div_either_side_of_half() { ticket_5_test_inner::rounded_div_either_side_of_half(); }

/** 契约：src/lib.rs:120-126,185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 负除数按商的符号舍入
 *   Given D=10 的 i64 原始计数 1 与 -4，或 -1 与 -4
 *   When 调用 rounded_div
 *   Then 分别读回 -3 或 3，操作数不变
 */
#[test]
fn rounded_div_negative_divisor() { ticket_5_test_inner::rounded_div_negative_divisor(); }

/** 契约：src/lib.rs:112-126,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 舍入乘除遇精确整数时不改变结果
 *   Given D=10 的 u64 原始计数 2 与 5，或 i64 原始计数 -2 与 5
 *   When 分别调用 rounded_mul 与 rounded_div
 *   Then 乘法读回 1 或 -1，除法读回 4 或 -4，操作数不变
 */
#[test]
fn rounded_exact_results() { ticket_5_test_inner::rounded_exact_results(); }

/** 契约：src/lib.rs:120-126,17-21,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 舍入除法遇零除数返回错误
 *   Given D=10 的 u64 原始计数 1 与 0，或 i64 原始计数 -1 与 0
 *   When 调用 rounded_div
 *   Then 返回 DivisionByZero，操作数不变
 */
#[test]
fn rounded_division_by_zero() { ticket_5_test_inner::rounded_division_by_zero(); }

/** 契约：src/lib.rs:112-126,17-21,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 舍入版乘除的最终计数超界
 *   Given D=10 的 u64::MAX 与 11（乘法）、u64::MAX 与 1（除法），或 i64::MIN 与 -10（乘法）
 *   When 分别调用 rounded_mul 或 rounded_div
 *   Then 返回 Overflow，操作数不变
 */
#[test]
fn rounded_arithmetic_overflows() { ticket_5_test_inner::rounded_arithmetic_overflows(); }

/** 契约：src/lib.rs:68-73,112-118,17-21,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 乘法截断仍在边界内但舍入后超界
 *   Given D=10 的 u64 原始计数 15372286728091293013 与 12、i64 原始计数 6148914691236517205 与 15，或 -8384883669867978008 与 11
 *   When 分别调用 checked_mul 与 rounded_mul
 *   Then 截断分别读回 u64::MAX、i64::MAX 或 i64::MIN；舍入返回 Overflow，操作数不变
 */
#[test]
fn rounded_mul_crosses_final_bound() { ticket_5_test_inner::rounded_mul_crosses_final_bound(); }

/** 契约：src/lib.rs:75-81,120-126,17-21,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 除法截断仍在边界内但舍入后超界
 *   Given D=10 的 u64 原始计数 12912720851596686131 与 7、i64 原始计数 3689348814741910323 与 4，或 -6456360425798343066 与 7
 *   When 分别调用 checked_div 与 rounded_div
 *   Then 截断分别读回 u64::MAX、i64::MAX 或 i64::MIN；舍入返回 Overflow，操作数不变
 */
#[test]
fn rounded_div_crosses_final_bound() { ticket_5_test_inner::rounded_div_crosses_final_bound(); }
