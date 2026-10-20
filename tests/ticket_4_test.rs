#[path = "support/ticket_4_test_inner.rs"]
mod ticket_4_test_inner;

/** 契约：src/lib.rs:68-73,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 普通乘法按同分母计算
 *   Given D=100 的 150 与 50（u64），或 -150 与 50（i64）
 *   When 调用 checked_mul
 *   Then 分别读回 75 或 -75，操作数不变
 */
#[test]
fn checked_mul_scales() { ticket_4_test_inner::checked_mul_scales(); }

/** 契约：src/lib.rs:75-81,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 普通除法按同分母计算
 *   Given D=100 的 150 与 50（u64），或 -150 与 50（i64）
 *   When 调用 checked_div
 *   Then 分别读回 300 或 -300，操作数不变
 */
#[test]
fn checked_div_scales() { ticket_4_test_inner::checked_div_scales(); }

/** 契约：src/lib.rs:75-81,185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 普通除法遇到非零负除数保留符号
 *   Given D=100 的 i64 原始计数 150 与 -50，或 -150 与 -50
 *   When 调用 checked_div
 *   Then 分别读回 -300 或 300，操作数不变
 */
#[test]
fn checked_div_negative_divisor() { ticket_4_test_inner::checked_div_negative_divisor(); }

/** 契约：src/lib.rs:68-81,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario: 默认乘法向零截断
 *   Given D=10 的 i64 原始计数 -1 与 5
 *   When 调用 checked_mul
 *   Then 读回 0，操作数不变
 */
#[test]
fn checked_mul_truncates() { ticket_4_test_inner::checked_mul_truncates(); }

/** 契约：src/lib.rs:75-81,185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario: 默认除法向零截断
 *   Given D=10 的 i64 原始计数 -1 与 4
 *   When 调用 checked_div
 *   Then 读回 -2，操作数不变
 */
#[test]
fn checked_div_truncates() { ticket_4_test_inner::checked_div_truncates(); }

/** 契约：src/lib.rs:68-81,17-21,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 普通乘法最终结果超界
 *   Given D=10 的 u64::MAX 与 11、i64::MAX 与 11，或 i64::MIN 与 11
 *   When 调用 checked_mul
 *   Then 均返回 Overflow，操作数不变
 */
#[test]
fn checked_mul_overflows() { ticket_4_test_inner::checked_mul_overflows(); }

/** 契约：src/lib.rs:75-81,17-21,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 普通除法最终商超界
 *   Given D=10 的 u64::MAX、i64::MAX 或 i64::MIN，除数均为 1
 *   When 调用 checked_div
 *   Then 均返回 Overflow，操作数不变
 */
#[test]
fn checked_div_overflows() { ticket_4_test_inner::checked_div_overflows(); }

/** 契约：src/lib.rs:97-110,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario: 未超界的饱和乘法保留精确结果
 *   Given D=100 的 u64 原始计数 150 与 50
 *   When 调用 saturating_mul
 *   Then 读回 75，操作数不变
 */
#[test]
fn saturating_mul_fits() { ticket_4_test_inner::saturating_mul_fits(); }

/** 契约：src/lib.rs:104-110,184
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario: 未超界的饱和除法保留精确结果
 *   Given D=100 的 u64 原始计数 150 与 50
 *   When 调用 saturating_div
 *   Then 读回 300，操作数不变
 */
#[test]
fn saturating_div_fits() { ticket_4_test_inner::saturating_div_fits(); }

/** 契约：src/lib.rs:104-110,185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 饱和除法遇到非零负除数保留符号
 *   Given D=100 的 i64 原始计数 150 与 -50，或 -150 与 -50
 *   When 调用 saturating_div
 *   Then 分别读回 -300 或 300，操作数不变
 */
#[test]
fn saturating_div_negative_divisor() { ticket_4_test_inner::saturating_div_negative_divisor(); }

/** 契约：src/lib.rs:97-110,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 饱和乘法将超界结果夹到相应边界
 *   Given D=10 的 u64::MAX 与 11、i64::MAX 与 11，或 i64::MIN 与 11
 *   When 调用 saturating_mul
 *   Then 分别读回 u64::MAX、i64::MAX 或 i64::MIN，操作数不变
 */
#[test]
fn saturating_mul_clamps() { ticket_4_test_inner::saturating_mul_clamps(); }

/** 契约：src/lib.rs:104-110,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 非零除数的饱和除法夹到相应边界
 *   Given D=10 的 u64::MAX、i64::MAX 或 i64::MIN，除数均为 1
 *   When 调用 saturating_div
 *   Then 分别读回 u64::MAX、i64::MAX 或 i64::MIN，操作数不变
 */
#[test]
fn saturating_div_clamps() { ticket_4_test_inner::saturating_div_clamps(); }

/** 契约：src/lib.rs:75-81,104-110,17-21,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 普通除零返回 DivisionByZero
 *   Given D=10 的 u64::MAX 与 0，或 i64 的 -15 与 0
 *   When 调用 checked_div
 *   Then 均返回 DivisionByZero，操作数不变
 */
#[test]
fn checked_division_by_zero() { ticket_4_test_inner::checked_division_by_zero(); }

/** 契约：src/lib.rs:104-110,17-21,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 饱和除零也返回 DivisionByZero
 *   Given D=10 的 u64::MAX 与 0，或 i64 的 -15 与 0
 *   When 调用 saturating_div
 *   Then 均返回 DivisionByZero，操作数不变
 */
#[test]
fn saturating_division_by_zero() { ticket_4_test_inner::saturating_division_by_zero(); }
