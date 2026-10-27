#![cfg(feature = "math")]
#[path = "support/ticket_12_test_inner.rs"]
mod ticket_12_test_inner;

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片12；01-requirements/needs.md 两档指数、对数和整数次幂（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_12_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @critical
 * Scenario Outline: powers_match_independent_integer_oracle
 * Given D=8、97或u32::MAX，raw为正负1、D/2、D-1、D、D+1、2D，指数-3至3；无符号只取正raw
 * When 分别求普通及严格整数幂
 * Then 精确raw以整数有理式raw^n/D^(n-1)或D^(1-n)/raw^(-n)独立求半值远离零；超界报Overflow，普通成功偏差至多1，严格成功必须等于基准；严格可显式PrecisionExhausted但不能猜测；源raw不变。
 */
#[test]
fn powers_match_independent_integer_oracle() {
    ticket_12_test_inner::powers_match_independent_integer_oracle();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片12、最终raw范围合同（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_12_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario: near_half_cubic_has_independent_64_bit_answer
 * Given 无符号D4113749459、raw5483946715995、指数3，精确raw的余数为(D²+1)/2
 * When 求普通及严格powi(3)
 * Then 整数基准为9745497238343526015；严格精确相等，普通偏差至多1，源raw不变，结果仍为同分母无符号值。
 */
#[test]
fn near_half_cubic_has_independent_64_bit_answer() {
    ticket_12_test_inner::near_half_cubic_has_independent_64_bit_answer();
}
