#![cfg(feature = "math")]

#[path = "support/ticket_13_test_inner.rs"]
mod ticket_13_test_inner;

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @critical
 * Scenario Outline: seven_methods_preserve_type_and_scale
 *   Given 两类D100输入0、1、8、100、2；When 分别调用fast_exp(0)、fast_exp2(0)、fast_ln(1)、fast_log2(8)、fast_log10(100)、fast_log(100,10)、fast_powi(2,-1)；Then 同类别D100的raw依次100、100、0、300、200、200、50，源值不变。
 */
#[test]
fn seven_methods_preserve_type_and_scale() {
    ticket_13_test_inner::seven_methods_preserve_type_and_scale();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: nondecimal_scales_and_maximum_denominator
 *   Given 两类D3的零、三分之一、二，以及D=u32::MAX的零；When 调用fast_exp或fast_powi(-1)；Then exp(0)为D raw、D3的exp(1/3)为4 raw、D3的2^-1为2 raw，返回原类别分母。
 */
#[test]
fn nondecimal_scales_and_maximum_denominator() {
    ticket_13_test_inner::nondecimal_scales_and_maximum_denominator();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: binary_exact_half_ties_round_away
 *   Given 有符号D10 raw±15或±20，无符号取正值；When 对±1.5求平方或立方、对±2求-2或-3次幂；Then raw分别为23、±34、3、±1（平方与-2幂为正），精确二进制半值远离零。
 */
#[test]
fn binary_exact_half_ties_round_away() {
    ticket_13_test_inner::binary_exact_half_ties_round_away();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: zero_powers_and_minimum_exponent
 *   Given 两类D100的0、1及有符号-1；When 求0^0、任意值零次幂、±1的i32::MIN次幂和-1的i32::MAX次幂；Then 零次幂及偶次幂为100 raw，负一奇次幂为-100 raw，无整数取绝对值溢出。
 */
#[test]
fn zero_powers_and_minimum_exponent() {
    ticket_13_test_inner::zero_powers_and_minimum_exponent();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: invalid_log_inputs_are_domain_errors
 *   Given 有符号D100 raw0、-100和无符号raw0；When 调用fast_ln、fast_log2、fast_log10和以2为底fast_log；Then 全部Domain且原输入不变。
 */
#[test]
fn invalid_log_inputs_are_domain_errors() {
    ticket_13_test_inner::invalid_log_inputs_are_domain_errors();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: invalid_bases_are_domain_errors
 *   Given D100正输入2，底数0、1或有符号-2；When fast_log；Then Domain且两操作数不变。
 */
#[test]
fn invalid_bases_are_domain_errors() {
    ticket_13_test_inner::invalid_bases_are_domain_errors();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: zero_negative_powers_are_domain_errors
 *   Given 两类D100零；When fast_powi(-1)或fast_powi(i32::MIN)；Then Domain且零输入不变。
 */
#[test]
fn zero_negative_powers_are_domain_errors() {
    ticket_13_test_inner::zero_negative_powers_are_domain_errors();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @critical
 * Scenario Outline: signed_logarithms_cover_base_quadrants
 *   Given 有符号D100输入4或0.25及底数2或0.5；When fast_log；Then 四组raw分别200、-200、-200、200，两操作数不变。
 */
#[test]
fn signed_logarithms_cover_base_quadrants() {
    ticket_13_test_inner::signed_logarithms_cover_base_quadrants();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: unsigned_negative_quantizes_before_range_check
 *   Given 无符号D2输入raw1、底数raw200以及D100输入raw50；When 前者fast_log、后者fast_ln；Then 微小负log量化为raw0成功，而ln(0.5)量化负raw返回Overflow。
 */
#[test]
fn unsigned_negative_quantizes_before_range_check() {
    ticket_13_test_inner::unsigned_negative_quantizes_before_range_check();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: storage_edges_survive_identity_power
 *   Given D1和D=u32::MAX的有符号MIN/MAX和无符号MAX；When fast_powi(1)；Then 原raw逐位保留且源值不变。
 */
#[test]
fn storage_edges_survive_identity_power() {
    ticket_13_test_inner::storage_edges_survive_identity_power();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @edgecase
 * Scenario Outline: proven_exponential_extremes
 *   Given 两类D1输入101及有符号-101；When fast_exp或fast_exp2；Then 正值Overflow，负值raw0，源值不变。
 */
#[test]
fn proven_exponential_extremes() {
    ticket_13_test_inner::proven_exponential_extremes();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: finite_powers_outside_storage_overflow
 *   Given 两类D1输入2和有符号-2；When 2^64或(-2)^65；Then Overflow而不是PrecisionExhausted，源值不变。
 */
#[test]
fn finite_powers_outside_storage_overflow() {
    ticket_13_test_inner::finite_powers_outside_storage_overflow();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（设计未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: invalid_large_intermediate_is_not_reciprocated_to_zero
 *   Given 两类D1的最大raw；When fast_powi(i32::MIN)；Then 正幂超出后端指数表示范围，返回ComputationFailed，不将无穷取倒数静默返回零，源值不变。
 */
#[test]
fn invalid_large_intermediate_is_not_reciprocated_to_zero() {
    ticket_13_test_inner::invalid_large_intermediate_is_not_reciprocated_to_zero();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md「固定128 bit快速近似档」切片13（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:tests/ticket_13_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: contract
 * @negative
 * Scenario Outline: each_fast_method_rejects_zero_denominator
 * Given FixedI64<D>和FixedU64<D>，用合法整数零表示绕过构造器；D1作为同源编译阳性对照
 * When 独立编译fast_exp、fast_exp2、fast_ln、fast_log2、fast_log10、fast_log(base)、fast_powi(-1)各调用
 * Then D1的14个样例均编译通过；D0的14个样例均因fast_math分母常量断言E0080失败，而非构造器、导入或链接失败。
 */
#[test]
fn each_fast_method_rejects_zero_denominator() {
    ticket_13_test_inner::each_fast_method_rejects_zero_denominator();
}
