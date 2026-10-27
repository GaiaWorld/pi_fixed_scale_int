#![cfg(feature = "math")]

#[path = "support/ticket_10_test_inner.rs"]
mod ticket_10_test_inner;

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @critical
 * Scenario Outline: strict_transcendentals
 *   Given D1000000的两类输入exp(1)、exp2(0.5)、ln(2)、log2(3)、log10(2)；When 求对应严格方法；Then raw恰为2718282、1414214、693147、1584963、301030，返回同类别同分母且源值不变。
 */
#[test]
fn strict_transcendentals() {
    ticket_10_test_inner::strict_transcendentals();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: strict_rational_power_oracle
 *   Given D10 raw从-25到25、指数从-5到5（排除零的负幂），无符号取非负raw；When 求powi_strict；Then 成功raw必须精确等于独立i128有理数半值远离零基准；无法证明时仅允许PrecisionExhausted，其他错误失败；涵盖正负半值、倒数和零次幂，源值不变。
 */
#[test]
fn strict_rational_power_oracle() {
    ticket_10_test_inner::strict_rational_power_oracle();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @critical
 * Scenario Outline: strict_logarithm_quadrants
 *   Given 有符号D1000000输入4或0.25，底数2或0.5；When 求log_strict；Then 四组raw依次为2000000、-2000000、-2000000、2000000，两个源值不变。
 */
#[test]
fn strict_logarithm_quadrants() {
    ticket_10_test_inner::strict_logarithm_quadrants();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: strict_near_one_bases
 *   Given D=u32::MAX，底数raw=D-1或D+1，输入同底数或raw=D，两类均覆盖；When 求log_strict；Then 同底数得raw=D，输入一得raw=0，源值不变。
 */
#[test]
fn strict_near_one_bases() {
    ticket_10_test_inner::strict_near_one_bases();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: strict_maximum_denominator
 *   Given D=u32::MAX的两类值一，以及有符号D3的1/3；When 求exp_strict；Then raw分别为11674931552及4，类别分母及源值不变。
 */
#[test]
fn strict_maximum_denominator() {
    ticket_10_test_inner::strict_maximum_denominator();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: strict_storage_boundaries
 *   Given D1或D=u32::MAX有符号MIN/MAX及无符号MAX；When 求powi_strict(1)；Then 精确保留64位raw，不丢失低位，源值不变。
 */
#[test]
fn strict_storage_boundaries() {
    ticket_10_test_inner::strict_storage_boundaries();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: strict_zero_and_minimum_exponents
 *   Given D10两类raw0/1/20，以及有符号raw正负10、无符号raw10；When 前组求powi_strict(0)，后组求powi_strict(i32::MIN)；Then raw均恰为10，不二次缩放，源值不变。
 */
#[test]
fn strict_zero_and_minimum_exponents() {
    ticket_10_test_inner::strict_zero_and_minimum_exponents();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: strict_invalid_logarithms
 *   Given D10有符号raw-1/0及无符号raw0；When 求ln_strict、log2_strict、log10_strict、log_strict(2)；Then 均为Domain且源值不变。
 */
#[test]
fn strict_invalid_logarithms() {
    ticket_10_test_inner::strict_invalid_logarithms();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: strict_invalid_bases_and_zero_negative_power
 *   Given D10输入2、底数-1/0/1（无符号取0/1），另有两类零和指数-1或i32::MIN；When 求严格指定底数对数或零的负幂；Then 返回Domain，全部源值不变。
 */
#[test]
fn strict_invalid_bases_and_zero_negative_power() {
    ticket_10_test_inner::strict_invalid_bases_and_zero_negative_power();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: strict_unsigned_rounded_range
 *   Given 无符号D2的0.5和底数100，以及D100的0.5；When 前者求log_strict(100)，后者求严格ln/log2/log10；Then 前者微小负raw舍入为0而成功，后三者因舍入raw为-69/-100/-30返回Overflow，源值不变。
 */
#[test]
fn strict_unsigned_rounded_range() {
    ticket_10_test_inner::strict_unsigned_rounded_range();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: strict_exponential_extremes
 *   Given 有符号D1输入-101/-100/100/101及两类64位极值；When 求exp_strict及exp2_strict；Then 负极端恰为raw0，正极端Overflow，源值不变。
 */
#[test]
fn strict_exponential_extremes() {
    ticket_10_test_inner::strict_exponential_extremes();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及用户裁定后的交接合同；01-requirements/needs.md「两档指数、对数和整数次幂」「高精度计算的定义域与失败」（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_10_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: strict_binary_rounding_and_overflow
 *   Given D1有符号输入-2/-1/62/63及无符号输入63/64、MAX；When 前两组求exp2_strict且MAX求log2_strict；Then raw依次为0/1/4611686018427387904/Overflow、9223372036854775808/Overflow及64，源值不变。
 */
#[test]
fn strict_binary_rounding_and_overflow() {
    ticket_10_test_inner::strict_binary_rounding_and_overflow();
}
