#![cfg(feature = "math")]

#[path = "support/ticket_9_additional_test_inner.rs"]
mod ticket_9_additional_test_inner;

/** 契约：07-design/call-chain.md 切片9、有限区间组合证明；01-requirements/needs.md「两档指数、对数和整数次幂」（设计未分配API ID）
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @critical
 * Scenario Outline: negative_transcendentals_have_one_raw_error_bound
 *   Given 有符号D1000000，exp输入-1、exp2输入-0.5、ln输入0.5、log2输入0.75、log10输入0.5
 *   When 分别调用普通数学方法
 *   Then 同类型同分母raw分别距367879、707107、-693147、-415037、-301030不超过1，源raw不变
 */
#[test]
fn negative_transcendentals_have_one_raw_error_bound() {
    ticket_9_additional_test_inner::negative_transcendentals_have_one_raw_error_bound();
}

/** 契约：07-design/call-chain.md 切片9、有限区间组合证明；01-requirements/needs.md「舍入方式」「两档指数、对数和整数次幂」（设计未分配API ID）
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario: maximum_denominator_negative_exponential_half_tie
 *   Given 有符号D4294967295且raw=-4294967295，表示-1
 *   When 调用普通exp2
 *   Then 精确raw为2147483647.5，半值远离零基准2147483648，返回同类型同分母且偏差不超过1，源raw不变
 */
#[test]
fn maximum_denominator_negative_exponential_half_tie() {
    ticket_9_additional_test_inner::maximum_denominator_negative_exponential_half_tie();
}

/** 契约：07-design/call-chain.md「尚须落实的实现与回归保护」指数函数早退不可套用整数幂；切片9（设计未分配API ID）
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: powers_above_one_hundred_are_not_exponential_overflow
 *   Given 两类D1000 raw101000表示101，有符号另取raw-101000，指数1或-1
 *   When 调用普通powi
 *   Then 一次幂raw基准为原raw，负一次幂raw基准为10或-10，结果偏差不超过1且类型分母不变，不报指数函数的Overflow，源raw不变
 */
#[test]
fn powers_above_one_hundred_are_not_exponential_overflow() {
    ticket_9_additional_test_inner::powers_above_one_hundred_are_not_exponential_overflow();
}
