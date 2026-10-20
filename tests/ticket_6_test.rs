#[path = "support/ticket_6_test_inner.rs"]
mod ticket_6_test_inner;

/** 契约：src/lib.rs:128-142,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 换分母对正负半值采用截断或远离零舍入
 *   Given 分母 100、原始计数 125（u64/i64）、-125 或 -5（i64）
 *   When 分别换到分母 10，选择默认或半值远离零
 *   Then 目标原始计数分别为 12 或 13、-12 或 -13、0 或 -1，源计数不变
 */
#[test]
fn rescale_half_ties() { ticket_6_test_inner::rescale_half_ties(); }

/** 契约：src/lib.rs:128-142,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 换分母按半值两侧及整值决定是否进位
 *   Given 分母 100 的计数 124、126、120（u64）或相应负计数（i64），及最大 u32 分母下的计数 1 或 -1
 *   When 分别默认或舍入换到分母 10；最大分母下换到自身
 *   Then 124 与 120 得 12，126 截断得 12、舍入得 13；负数对应取负，自身换分母保留原计数，源值不变
 */
#[test]
fn rescale_around_half_and_exact() { ticket_6_test_inner::rescale_around_half_and_exact(); }

/** 契约：src/lib.rs:128-142,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 换到更大分母时普通计数正确放大
 *   Given 分母 10 的无符号原始计数 12，或有符号原始计数 12、-12
 *   When 分别以默认或半值远离零方式换到分母 100
 *   Then 目标原始计数分别为 120、120、-120，源计数不变
 */
#[test]
fn rescale_to_larger_denominator() { ticket_6_test_inner::rescale_to_larger_denominator(); }

/** 契约：src/lib.rs:17-21,128-142,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 换分母后计数超出 64 位范围
 *   Given 分母 1、原始计数 u64::MAX、i64::MAX 或 i64::MIN
 *   When 分别默认或舍入换到分母 2
 *   Then 均返回 Overflow，源计数不变
 */
#[test]
fn rescale_overflows() { ticket_6_test_inner::rescale_overflows(); }

/** 契约：src/lib.rs:17-21,128-142,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 换分母截断可表示但舍入后越界
 *   Given (D,TO,raw) 为 (2,31,1190112520884487201) 的 u64，(2,3,6148914691236517205) 或 (4,5,-7378697629483820647) 的 i64
 *   When 分别默认或舍入换分母
 *   Then 截断目标为 u64::MAX、i64::MAX 或 i64::MIN，舍入返回 Overflow，源计数不变
 */
#[test]
fn rescale_rounding_crosses_bound() { ticket_6_test_inner::rescale_rounding_crosses_bound(); }

/** 契约：src/lib.rs:128-142,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 零目标分母的换分母公开操作无法编译
 *   Given 分母 10 的数值及目标分母 0
 *   When 分别编译默认及舍入换分母调用
 *   Then 均因分母正值约束编译失败
 */
#[test]
fn zero_target_denominator_fails_to_compile() { ticket_6_test_inner::zero_target_denominator_fails_to_compile(); }

/** 契约：src/lib.rs:128-142,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 零源分母的换分母公开操作无法编译
 *   Given 已取得零分母的 u64 或 i64 值，目标分母为 10
 *   When 分别编译默认及舍入换分母调用
 *   Then 均因分母正值约束编译失败
 */
#[test]
fn zero_source_rescale_fails_to_compile() { ticket_6_test_inner::zero_source_rescale_fails_to_compile(); }

/** 契约：src/lib.rs:144-155,184-185
 * @critical
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 输出整数对正负半值截断或远离零舍入
 *   Given 分母 10、原始计数 15（u64/i64）、-15 或 -5（i64）
 *   When 分别截断或舍入输出整数
 *   Then 输出为 1 或 2、-1 或 -2、0 或 -1，源计数不变
 */
#[test]
fn integer_output_half_ties() { ticket_6_test_inner::integer_output_half_ties(); }

/** 契约：src/lib.rs:144-155,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 整数输出在半值两侧和精确值正确处理
 *   Given 分母 10 的计数 14、16、20 或相应负计数
 *   When 分别截断或舍入输出整数
 *   Then 14 得正负 1；16 截断得正负 1、舍入得正负 2；20 得正负 2，源计数不变
 */
#[test]
fn integer_output_around_half_and_exact() { ticket_6_test_inner::integer_output_around_half_and_exact(); }

/** 契约：src/lib.rs:144-155,184-185
 * @edgecase
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 整数输出不扩大范围
 *   Given 分母 1、原始计数 u64::MAX、i64::MAX 或 i64::MIN
 *   When 分别截断或舍入输出整数
 *   Then 返回对应原始计数，源计数不变
 */
#[test]
fn integer_output_extremes() { ticket_6_test_inner::integer_output_extremes(); }

/** 契约：src/lib.rs:144-155,184-185
 * @negative
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * Scenario Outline: 零源分母的整数输出公开操作无法编译
 *   Given 已取得零分母的 u64 或 i64 值
 *   When 分别编译截断及舍入整数输出调用
 *   Then 均因分母正值约束编译失败
 */
#[test]
fn zero_source_integer_output_fails_to_compile() { ticket_6_test_inner::zero_source_integer_output_fails_to_compile(); }
