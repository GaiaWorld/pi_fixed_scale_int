#![cfg(feature = "math")]

#[path = "support/ticket_9_test_inner.rs"]
mod ticket_9_test_inner;

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @critical
 * Scenario Outline: ordinary_transcendentals
 *   Given D1000000两类值，exp输入1、exp2输入0.5、ln输入2、log2输入3、log10输入2
 *   When 调用对应普通数学方法
 *   Then 保持类型分母，raw分别距2718282/1414214/693147/1584963/301030不超过1，源值不变
 */
#[test]
fn ordinary_transcendentals() {
    ticket_9_test_inner::ordinary_transcendentals();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @critical
 * Scenario Outline: logarithm_sign_quadrants
 *   Given D1000000有符号输入4/0.25，底数2/0.5
 *   When 求普通log(base)
 *   Then 四组raw分别距2000000/-2000000/-2000000/2000000不超过1，两个操作数不变
 */
#[test]
fn logarithm_sign_quadrants() {
    ticket_9_test_inner::logarithm_sign_quadrants();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: rational_power_oracle
 *   Given D10带符号raw=-21/-20/-19/-5/0/5/19/20/21，无符号取非负项，指数=-3/-2/-1/1/2/3（排除零的负幂）
 *   When 求普通powi
 *   Then 距独立i128有理数半值远离零基准不超过1，类型分母源值不变，涵盖正负半值及倒数
 */
#[test]
fn rational_power_oracle() {
    ticket_9_test_inner::rational_power_oracle();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: zero_power_is_one
 *   Given 两类D10 raw0/1/20，两类D=u32::MAX raw0
 *   When 求powi(0)
 *   Then raw距D不超过1，而非D平方，源值不变
 */
#[test]
fn zero_power_is_one() {
    ticket_9_test_inner::zero_power_is_one();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @negative
 * Scenario Outline: invalid_logarithms
 *   Given 有符号D10 raw0/-1，无符号D10 raw0
 *   When 分别求ln/log2/log10及log(2)
 *   Then 均返回Domain，源值不变
 */
#[test]
fn invalid_logarithms() {
    ticket_9_test_inner::invalid_logarithms();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @negative
 * Scenario Outline: invalid_bases
 *   Given D10输入raw20，有符号底数raw-10/0/10，无符号底数raw0/10
 *   When 求log(base)
 *   Then 均返回Domain，两操作数不变
 */
#[test]
fn invalid_bases() {
    ticket_9_test_inner::invalid_bases();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @negative
 * Scenario Outline: zero_negative_power
 *   Given 两类D10的零，指数-1/i32::MIN
 *   When 求powi
 *   Then 均返回Domain且源raw仍0
 */
#[test]
fn zero_negative_power() {
    ticket_9_test_inner::zero_negative_power();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: unsigned_rounded_range
 *   Given 无符号D2 raw1和底数raw200；无符号D100 raw50
 *   When 前者求log(100)，后者分别求ln/log2/log10
 *   Then 前者精确raw约-0.30103舍入0，成功raw为0或1；后三者舍入raw=-69/-100/-30均报Overflow；源值不变
 */
#[test]
fn unsigned_rounded_range() {
    ticket_9_test_inner::unsigned_rounded_range();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: exponential_extremes
 *   Given 有符号D1 raw-101/-100/100/101，无符号D1 raw100/101
 *   When 分别求exp和exp2
 *   Then 负值成功raw为0或1，正值返回Overflow，源值不变
 */
#[test]
fn exponential_extremes() {
    ticket_9_test_inner::exponential_extremes();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: power_storage_boundaries
 *   Given D1有符号MIN/MAX及无符号MAX
 *   When 分别求powi(1)和powi(2)
 *   Then 一次幂距原raw不超过1，平方返回Overflow，源值不变
 */
#[test]
fn power_storage_boundaries() {
    ticket_9_test_inner::power_storage_boundaries();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: minimum_exponent
 *   Given D10有符号raw-10/10及无符号raw10
 *   When 求powi(i32::MIN)
 *   Then 结果raw距10不超过1，源值不变
 */
#[test]
fn minimum_exponent() {
    ticket_9_test_inner::minimum_exponent();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: maximum_denominator
 *   Given 两类D=u32::MAX的raw=D，以及有符号D3 raw1
 *   When 求exp
 *   Then 前两者raw距11674931552不超过1，后者距4不超过1，类型分母和源值不变
 */
#[test]
fn maximum_denominator() {
    ticket_9_test_inner::maximum_denominator();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片9及有限区间组合证明；design/pi-fixed-scale-int/01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: near_one_log_base
 *   Given 两类D4294967295输入raw4294967296，底数raw4294967296或4294967294
 *   When 求log(base)
 *   Then 相同底数raw距4294967295不超过1；异侧底数有符号raw距-4294967294不超过1、无符号报Overflow；两源值不变
 */
#[test]
fn near_one_log_base() {
    ticket_9_test_inner::near_one_log_base();
}
