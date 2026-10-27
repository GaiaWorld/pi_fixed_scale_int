#![cfg(feature = "math")]
#[path = "support/ticket_10_additional_test_inner.rs"]
mod ticket_10_additional_test_inner;

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: exact_binary_half_ties
 * Given 有符号D1 raw正负2指数-1、D10 raw正负5指数3，以及对应非负无符号输入；When 求powi_strict；Then 精确raw分别为正负1，不能向零舍入，源值不变。
 */
#[test]
fn exact_binary_half_ties() { ticket_10_additional_test_inner::exact_binary_half_ties(); }

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: power_overflow
 * Given D1有符号MIN/MAX及无符号MAX；When 求powi_strict(2)；Then 返回Overflow且源raw不变。
 */
#[test]
fn power_overflow() { ticket_10_additional_test_inner::power_overflow(); }
