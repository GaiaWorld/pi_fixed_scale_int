#![cfg(feature = "math")]

#[path = "support/ticket_9_power_edges_test_inner.rs"]
mod ticket_9_power_edges_test_inner;

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9、有限区间组合证明第3/7项；01-requirements/needs.md「高精度计算的定义域与失败」（未分配API ID）
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @negative
 * Scenario Outline: signed_odd_powers_below_minimum_overflow
 *   Given 有符号D1 raw=-2且指数65，或raw=i64::MIN且指数3
 *   When 调用普通powi
 *   Then 精确raw分别为-2^65、-2^189，均小于i64::MIN，返回Overflow且源raw不变
 */
#[test]
fn signed_odd_powers_below_minimum_overflow() {
    ticket_9_power_edges_test_inner::signed_odd_powers_below_minimum_overflow();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9、有限区间组合证明第3/7项；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario: signed_odd_power_at_minimum_succeeds
 *   Given 有符号D1 raw=-2，指数63
 *   When 调用普通powi
 *   Then 数学raw=-9223372036854775808可表示，成功返回同类型同分母，raw距该基准不超过1且不越界，源raw仍为-2
 */
#[test]
fn signed_odd_power_at_minimum_succeeds() {
    ticket_9_power_edges_test_inner::signed_odd_power_at_minimum_succeeds();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及「尚须落实的实现与回归保护」早退限制；01-requirements/needs.md「两档指数、对数和整数次幂」（未分配API ID）
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: large_integer_exponents_preserve_unit_magnitude
 *   Given D1000有符号raw=-1000/1000，无符号raw=1000，指数101/-101/102/-102
 *   When 调用普通powi
 *   Then 负底数奇数指数数学raw=-1000，其余数学raw=1000；均成功且偏差不超过1，类型分母和源raw不变，不能因指数大于100报Overflow或小于-100返回零
 */
#[test]
fn large_integer_exponents_preserve_unit_magnitude() {
    ticket_9_power_edges_test_inner::large_integer_exponents_preserve_unit_magnitude();
}
