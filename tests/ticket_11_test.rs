#![cfg(feature = "math")]
#[path = "support/ticket_11_test_inner.rs"]
mod ticket_11_test_inner;

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片11、有限区间组合证明第3项（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_11_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @critical
 * Scenario Outline: negative_powers_preserve_sign_and_scale
 * Given D8有符号raw正负4、正负16，以及对应正值无符号输入，指数-3或-2
 * When 调用普通或严格powi
 * Then 独立有理基准raw分别为正负64/32与正负1/2（偶次幂为正）；严格等于基准，普通偏差至多1，源raw不变。
 */
#[test]
fn negative_powers_preserve_sign_and_scale() {
    ticket_11_test_inner::negative_powers_preserve_sign_and_scale();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片11、无穷处理合同（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_11_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: huge_negative_power_never_returns_false_zero
 * Given D1有符号及无符号最大raw，指数i32::MIN
 * When 调用普通或严格powi
 * Then 正幂中间值超出后端指数范围，返回ComputationFailed而非倒数伪精确零，源raw不变。
 */
#[test]
fn huge_negative_power_never_returns_false_zero() {
    ticket_11_test_inner::huge_negative_power_never_returns_false_zero();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 最终raw范围及指数早退证明（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_11_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: proven_exponential_extremes_respect_max_denominator
 * Given 有符号D=u32::MAX，数值正负101
 * When 调用普通或严格exp、exp2
 * Then 正值返回Overflow，负值正确舍入raw0，源raw不变。
 */
#[test]
fn proven_exponential_extremes_respect_max_denominator() {
    ticket_11_test_inner::proven_exponential_extremes_respect_max_denominator();
}

/** 契约：design/pi-fixed-scale-int/01-requirements/needs.md 高精度计算的定义域与失败（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_11_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario: unsigned_range_is_decided_after_rounding
 * Given 无符号D2 raw1、底数raw200（数值100），以及D100 raw50
 * When 分别调用严格指定底数对数、严格自然对数
 * Then 前者精确raw约-0.30103舍入为0可成功，后者舍入raw为-69返回Overflow，两个源值及底数不变。
 */
#[test]
fn unsigned_range_is_decided_after_rounding() {
    ticket_11_test_inner::unsigned_range_is_decided_after_rounding();
}
