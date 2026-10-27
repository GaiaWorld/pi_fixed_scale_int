#![cfg(feature = "math")]

#[path = "support/ticket_14_test_inner.rs"]
mod ticket_14_test_inner;

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片14（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_14_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @critical
 * Scenario Outline: documented_fast_examples_keep_category_and_denominator
 *   Given 两类D100的0、1、100、10、2及有符号D10的-2；When 分别求exp2(0)、ln(1)、log(100,10)、powi(2,-1)与powi(-2,3)；Then raw依次100、0、200、50与-80，结果类型分母不变且源值不变。
 */
#[test]
fn documented_fast_examples_keep_category_and_denominator() {
    ticket_14_test_inner::documented_fast_examples_keep_category_and_denominator();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片14（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_14_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @critical
 * Scenario Outline: representative_values_use_independent_decimal_references
 *   Given 两类D1000000的1、0.5、2、3；When 分别求fast_exp(1)、fast_exp2(0.5)、fast_ln(2)、fast_log2(3)、fast_log10(2)、fast_log(3,2)、fast_powi(1.5,3)；Then 代表性且远离半值的raw依次2718282、1414214、693147、1584963、301030、1584963、3375000，不使用其他精度档作期望。
 */
#[test]
fn representative_values_use_independent_decimal_references() {
    ticket_14_test_inner::representative_values_use_independent_decimal_references();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片14（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_14_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: binary_rational_powers_match_independent_integer_oracle
 *   Given 有符号D8的raw -24、-20、-12、-4、4、12、20、24及无符号正值，指数-5到5；When fast_powi；Then 按精确整数分子分母独立计算raw并半值远离零舍入所得值一致，涵盖负底数、倒数、非十进制分母、正负半值，源值不变。
 */
#[test]
fn binary_rational_powers_match_independent_integer_oracle() {
    ticket_14_test_inner::binary_rational_powers_match_independent_integer_oracle();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片14（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_14_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: fast_domain_errors_preserve_operands
 *   Given 两类D100的0、1、2及有符号-1；When 求非正输入的fast_ln/log2/log10/log、以0或1或负数为底的fast_log、零的i32::MIN次幂；Then 均返回Domain，全部输入和底数不变。
 */
#[test]
fn fast_domain_errors_preserve_operands() {
    ticket_14_test_inner::fast_domain_errors_preserve_operands();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片14（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_14_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: fast_range_errors_preserve_operands
 *   Given 两类D1的2及无符号D100的0.5；When 求fast_powi(64)或fast_ln(0.5)；Then 均返回Overflow而非PrecisionExhausted，输入不变。
 */
#[test]
fn fast_range_errors_preserve_operands() {
    ticket_14_test_inner::fast_range_errors_preserve_operands();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片14（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_14_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: fast_invalid_intermediate_errors_preserve_operands
 *   Given D1有符号i64::MAX及无符号u64::MAX；When fast_powi(i32::MIN)；Then 失效中间幂返回ComputationFailed而非静默取倒数成零或PrecisionExhausted，输入不变。
 */
#[test]
fn fast_invalid_intermediate_errors_preserve_operands() {
    ticket_14_test_inner::fast_invalid_intermediate_errors_preserve_operands();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片14（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_14_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: fast_nonidentity_odd_power_at_signed_minimum_succeeds
 *   Given 有符号D1的raw -2097152及-2；When 分别fast_powi(3)及fast_powi(63)；Then 独立i128整数幂均为-9223372036854775808，返回同类别D1的i64::MIN且输入不变。
 */
#[test]
fn fast_nonidentity_odd_power_at_signed_minimum_succeeds() {
    ticket_14_test_inner::fast_nonidentity_odd_power_at_signed_minimum_succeeds();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片14（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_14_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: fast_adjacent_odd_power_below_signed_minimum_overflows
 *   Given 有符号D1的raw -2097153（合法下界立方底数-2097152的相邻整数）及-2；When 分别fast_powi(3)及fast_powi(65)；Then 独立i128整数幂均低于i64::MIN，返回Overflow且输入不变。
 */
#[test]
fn fast_adjacent_odd_power_below_signed_minimum_overflows() {
    ticket_14_test_inner::fast_adjacent_odd_power_below_signed_minimum_overflows();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片14（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_14_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @critical
 * Scenario Outline: ordinary_and_strict_still_match_independent_references
 *   Given 两类D1000000的1、0.5、2、3、1.5；When 调用七个普通及严格方法，分别求exp(1)、exp2(0.5)、ln(2)、log2(3)、log10(2)、log(3,2)、powi(1.5,3)；Then 严格raw依次2718282、1414214、693147、1584963、301030、1584963、3375000，普通与各独立期望差至多1 raw，输入不变。
 */
#[test]
fn ordinary_and_strict_still_match_independent_references() {
    ticket_14_test_inner::ordinary_and_strict_still_match_independent_references();
}
