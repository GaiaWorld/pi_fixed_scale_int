/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10–12及本轮私有预算入口授权（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能指定预算或读取真实循环的每轮精度。
 * @edgecase
 * Scenario Outline: real_budget_exhaustion_does_not_guess_half
 * Given D10 raw正负8的倒数，精确raw=正负12.5，严格预算从128开始，上限128或256
 * When 通过生产共用预算入口求值并读取真实事件
 * Then 均为PrecisionExhausted；实际尝试依次为[128]或[128,256]且仅记录一次最终错误，不把半值猜为正负13。
 */
#[test]
fn real_budget_exhaustion_does_not_guess_half() {
    math_inner_test_inner::real_budget_exhaustion_does_not_guess_half();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 两档精度及本轮默认预算不变合同（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能观察真实循环初始精度及最终结果事件。
 * @critical
 * Scenario Outline: production_entries_keep_default_precision
 * Given D1000000两类输入1
 * When 分别经公开普通exp和严格exp求值并读取事件
 * Then 严格raw为2718282、普通raw与2718282相差至多1；普通实际尝试[128]、严格实际尝试[192]，最终事件与返回值一致，源raw不变。
 */
#[test]
fn production_entries_keep_default_precision() {
    math_inner_test_inner::production_entries_keep_default_precision();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 用户裁定后的错误及最终raw范围合同（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能限制实际循环预算，故验证私有预算入口错误分类未被耗尽吞并。
 * @negative
 * Scenario Outline: limited_budget_preserves_error_categories
 * Given 严格预算128至256、ln(0)、D1的2^64（u64范围）、u64最大值的i32::MIN次幂，以及无符号D2的log100(0.5)
 * When 通过共用预算入口求值
 * Then 分别为Domain、Overflow、ComputationFailed及raw0，且每次仅记录一次对应最终结果。
 */
#[test]
fn limited_budget_preserves_error_categories() {
    math_inner_test_inner::limited_budget_preserves_error_categories();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 有限区间组合证明第4、7项（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: unit
 * @seam: inner
 * 公共API不能注入有限极端区间以触发单个四角商的底层异常。
 * @negative
 * Scenario Outline: corner_failure_is_not_hidden_by_aggregation
 * Given 有效有限分子[0,huge]或[-huge,0]，huge指数为依赖EXPONENT_MAX；分母[tiny,tiny]，tiny指数EXPONENT_MIN+128
 * When 四角区间除法，其中零商有限而极端商指数溢出
 * Then 返回ComputationFailed，不得由min/max隐藏异常商后返回成功。
 */
#[test]
fn corner_failure_is_not_hidden_by_aggregation() {
    math_inner_test_inner::corner_failure_is_not_hidden_by_aggregation();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10–12及真实预算循环配对验收授权（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能指定精度预算或观察循环的实际精度及最终结果事件。
 * @critical
 * Scenario: strict_cubic_converges_only_after_precision_doubles
 * Given 无符号范围、D=4113749459、raw=5483946715995、指数3；精确结果raw=raw³/D²，其余数为(D²+1)/2，独立整数舍入为9745497238343526015
 * When 同一输入经生产共用compute_with_budget严格求值，起始128，上限分别128和256
 * Then 首次预算返回PrecisionExhausted且实际尝试[128]；扩大预算返回raw9745497238343526015且实际尝试[128,256]；每次尝试均有真实evaluate事件，每次调用恰有一个与返回值相同的末尾PrecisionResult。
 */
#[test]
fn strict_cubic_converges_only_after_precision_doubles() {
    math_inner_test_inner::strict_cubic_converges_only_after_precision_doubles();
}

#[path = "math_inner_test_inner.rs"]
mod math_inner_test_inner;

/** 契约：票据12对数商分母跨零真实循环重试及预算耗尽授权（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 合法输入及最低128 bit精度不能自然构造跨零分母，须在实际evaluate内用测试队列放宽真实分母。
 * @edgecase
 * Scenario Outline: logarithm_crossing_retries_or_exhausts_real_budget
 * Given raw4、D2、底数raw4，数学log2(2)=1即raw2；测试仅在128轮或128及256轮将ln(2)区间放宽为[-1,1]
 * When 普通及严格compute_with_budget从128开始，上限128或256
 * Then 单轮预算耗尽且尝试[128]；仅128轮放宽时256预算尝试[128,256]并返回raw2；两轮均放宽时256预算耗尽且尝试[128,256]；队列全部消费，每轮真实evaluate且仅一条最终结果。
 */
#[test]
fn logarithm_crossing_retries_or_exhausts_real_budget() {
    math_inner_test_inner::logarithm_crossing_retries_or_exhausts_real_budget();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片12、指定底数定义域及区间证明（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能观察对数分母端点或限制真实循环预算；本场景是跨零不可达分析的边界佐证，不冒充跨零覆盖。
 * @edgecase
 * Scenario Outline: closest_legal_log_bases_stay_strictly_off_zero
 * Given D=u32::MAX、底数raw=D-1或D+1，这是合法底数距一最近的两侧值；输入等于底数，精度128或256
 * When 构造真实ratio及有向ln端点，并通过普通及严格预算循环求log
 * Then 下侧ln上下界均负，上侧均正；每个预算循环只尝试[128]，严格raw=D、普通距D至多1；底数raw=D经循环立即Domain且无尝试。
 */
#[test]
fn closest_legal_log_bases_stay_strictly_off_zero() {
    math_inner_test_inner::closest_legal_log_bases_stay_strictly_off_zero();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片12、负幂零端点重试合同（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共入口不能限制预算或观察evaluate错误引发的真实重试轮次。
 * @edgecase
 * Scenario Outline: reciprocal_zero_endpoint_retries_real_loop_until_budget_exhausts
 * Given raw1、D4、指数i32::MIN，正幂中间量为2^(-4294967296)，低于后端指数范围；预算128或256
 * When 普通或严格compute_with_budget从128开始调用真实evaluate
 * Then 每轮倒数前零端点触发PrecisionExhausted，真实尝试分别[128]和[128,256]，最终PrecisionExhausted；不执行倒数、四角聚合或量化，且仅一个最终结果事件。
 */
#[test]
fn reciprocal_zero_endpoint_retries_real_loop_until_budget_exhausts() {
    math_inner_test_inner::reciprocal_zero_endpoint_retries_real_loop_until_budget_exhausts();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片12、两档量化及真实预算循环授权（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能设置相同起始预算并读取实际升精度轮次。
 * @critical
 * Scenario: ordinary_and_strict_diverge_then_strict_converges
 * Given D4113749459、raw5483946715995、指数3、完整u64范围；整数基准9745497238343526015
 * When 普通与严格共用循环从128开始，预算分别128和256
 * Then 普通128成功且误差至多1、尝试[128]；严格128为PrecisionExhausted、尝试[128]；严格256精确等于基准、尝试[128,256]；每轮均有真实evaluate且末尾恰有一个对应结果事件。
 */
#[test]
fn ordinary_and_strict_diverge_then_strict_converges() {
    math_inner_test_inner::ordinary_and_strict_diverge_then_strict_converges();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 范围跨界重试合同（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能设置目标范围边界与精度预算或观察实际尝试。
 * @edgecase
 * Scenario Outline: range_straddling_retries_before_overflow
 * Given D4113749459，raw正负5483946715995，指数3，独立正确舍入raw正负9745497238343526015；正值min设为该raw，负值max设为该raw
 * When 普通及严格真实预算循环分别使用128和256上限
 * Then 128区间量化跨目标范围边界，返回PrecisionExhausted而非Overflow；256依次尝试[128,256]并成功返回独立基准，每次仅一条最终事件。
 */
#[test]
fn range_straddling_retries_before_overflow() {
    math_inner_test_inner::range_straddling_retries_before_overflow();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片11底层错误传播（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能注入携带底层错误的上下端点。
 * @negative
 * Scenario Outline: backend_error_endpoints_are_explicitly_rejected
 * Given 显式携带InvalidArgument的NaN端点，已确认err为Some(InvalidArgument)，分别位于lo及hi
 * When 校验区间、倒数前正区间检查，或作为四角除法分子及分母
 * Then 全部返回ComputationFailed而非PrecisionExhausted。
 */
#[test]
fn backend_error_endpoints_are_explicitly_rejected() {
    math_inner_test_inner::backend_error_endpoints_are_explicitly_rejected();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片11前置校验顺序（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API无法观察实际min/max和reciprocal执行点，需实现侧测试事件。
 * @negative
 * Scenario: validation_precedes_aggregation_and_reciprocal
 * Given 有效[2,4]/[2,4]与D8 raw16的-3次幂为阳性对照；异常四角商、错误端点及D1 raw=u64::MAX指数i32::MIN为阴性对照
 * When 调用真实divide或evaluate并读取线程隔离事件
 * Then 阳性分别出现divide.aggregate、power.reciprocal事件；异常角商和错误端点在聚合前返回ComputationFailed且无聚合事件；无效正幂在倒数前返回ComputationFailed且无倒数事件。
 */
#[test]
fn validation_precedes_aggregation_and_reciprocal() {
    math_inner_test_inner::validation_precedes_aggregation_and_reciprocal();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片11（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能注入异常区间，无法直接观察聚合前及倒数前校验。
 * @negative
 * Scenario Outline: invalid_endpoints_fail_before_division_or_reciprocal
 * Given NaN、正负无穷分别位于上下端点，或倒置[2,1]，另一区间为[1,2]
 * When 分别作为四角除法的分子、分母或倒数前正区间进行校验
 * Then 全部返回ComputationFailed，不能被聚合隐藏或误归类PrecisionExhausted。
 */
#[test]
fn invalid_endpoints_fail_before_division_or_reciprocal() {
    math_inner_test_inner::invalid_endpoints_fail_before_division_or_reciprocal();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 有限区间组合证明第4项（未分配API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能指定四角除法的封闭端点，无法检查全部角点的覆盖。
 * @edgecase
 * Scenario Outline: division_covers_all_corner_signs
 * Given 分子[2,4]、[-4,-2]或[-2,4]与分母[2,4]或[-4,-2]
 * When 四角区间除法
 * Then 依次得到[0.5,2]、[-2,-0.5]、[-2,-0.5]、[0.5,2]、[-1,2]、[-2,1]，端点有序且精确。
 */
#[test]
fn division_covers_all_corner_signs() {
    math_inner_test_inner::division_covers_all_corner_signs();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: unit
 * @seam: inner
 * 无公共API允许注入区间端点或独立调用量化、倒数保护，故仅测试对应计算芯。
 * @edgecase
 * Scenario Outline: invalid_intervals
 * Given 下界2上界1的倒置区间、0除0的NaN端点、1除0的无穷端点；When 校验区间；Then 均返回ComputationFailed，不进入聚合。
 */
#[test]
fn invalid_intervals() {
    math_inner_test_inner::invalid_intervals();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: unit
 * @seam: inner
 * 无公共API允许注入区间端点或独立调用量化、倒数保护，故仅测试对应计算芯。
 * @edgecase
 * Scenario Outline: zero_crossing_divisor
 * Given 分子区间[1,2]、分母[-1,1]或[0,1]；When 四角区间除法；Then 返回PrecisionExhausted供外层升精度，不返回伪有限值。
 */
#[test]
fn zero_crossing_divisor() {
    math_inner_test_inner::zero_crossing_divisor();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: unit
 * @seam: inner
 * 无公共API允许注入区间端点或独立调用量化、倒数保护，故仅测试对应计算芯。
 * @edgecase
 * Scenario Outline: zero_before_reciprocal
 * Given 有效区间[0,1]；When 检查倒数前严格正端点；Then 返回PrecisionExhausted供外层升精度。
 */
#[test]
fn zero_before_reciprocal() {
    math_inner_test_inner::zero_before_reciprocal();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: unit
 * @seam: inner
 * 无公共API允许注入区间端点或独立调用量化、倒数保护，故仅测试对应计算芯。
 * @edgecase
 * Scenario Outline: exact_half_quantization
 * Given 精确端点正负0.5、正负1.5以及u64::MAX；When 量化；Then 得正负1、正负2及u64::MAX，不经过f64。
 */
#[test]
fn exact_half_quantization() {
    math_inner_test_inner::exact_half_quantization();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及精度上限规则（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: unit
 * @seam: inner
 * 公共API不能指定求值精度或读取封闭端点，无法直接检查半值区间的每档量化，故调用计算芯。
 * @edgecase
 * Scenario Outline: reciprocal_half_remains_uncertified
 * Given D10 raw正负8、指数-1，精确raw为正负12.5，精度192逐次加倍至12288
 * When 各档evaluate并独立量化端点，然后经实际compute求严格结果
 * Then 各区间包住精确半值；若所有档端点量化均不一致，compute必须返回PrecisionExhausted，不能猜测或降级；若某档已唯一量化，compute必须给正确raw正负13。
 */
#[test]
fn reciprocal_half_remains_uncertified() {
    math_inner_test_inner::reciprocal_half_remains_uncertified();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片10、有限区间组合证明及raw单位合同（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @kind: unit
 * @seam: inner
 * 公共API不允许指定单次求值精度或读取端点，故直接测试计算芯。
 * @edgecase
 * Scenario Outline: explicit_precision_preserves_raw_bounds
 * Given D1000000输入1的exp、输入2的ln及零的零次幂，指定中间计算精度128、192、384 bit（最终仍为64位raw）
 * When 每档evaluate后量化端点
 * Then exp两端均为2718282、ln两端均为693147、零次幂两端均为1000000；区间有效，不重复缩放。
 */
#[test]
fn explicit_precision_preserves_raw_bounds() {
    math_inner_test_inner::explicit_precision_preserves_raw_bounds();
}

/** 契约：票据13授权观测接缝；design/pi-fixed-scale-int/07-design/call-chain.md 切片13（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不暴露实际后端精度、舍入或自适应入口事件。
 * @critical
 * Scenario Outline: fast_requests_use_only_fixed_budget
 * Given FixedI64<3>与FixedU64<3>，exp/exp2输入raw1，ln/log2输入raw6，log10输入raw30，log输入raw12底数raw6，powi输入raw6指数-1
 * When 独立调用各fast方法并读取该次观测
 * Then 每次成功且后端事件非空，每个请求precision=128、rounding=ToEven、operation非空，无AdaptiveEntry。
 */
#[test]
fn fast_requests_use_only_fixed_budget() {
    math_inner_test_inner::fast_requests_use_only_fixed_budget();
}

/** 契约：票据13授权观测接缝与整数幂/范围/异常合同（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能观察正负幂、异常及越界分支实际使用的计算预算。
 * @edgecase
 * Scenario Outline: fast_branch_requests_never_adapt
 * Given D10有符号raw-20，D100两类raw100，D1两类raw2和两类最大raw
 * When 分别求(-2)^3、(-2)^-3、1^i32::MIN、2^64及最大raw^i32::MIN
 * Then 依次raw-80、raw-1、raw100、Overflow、ComputationFailed，每次后端事件非空且全部128/ToEven，无自适应入口。
 */
#[test]
fn fast_branch_requests_never_adapt() {
    math_inner_test_inner::fast_branch_requests_never_adapt();
}

/** 契约：票据13授权观测接缝阳性对照（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不暴露compute/evaluate是否被观测。
 * @critical
 * Scenario Outline: ordinary_and_strict_entries_are_observable
 * Given 两类D100输入1
 * When 分别调用普通exp2与严格exp2并独立读取观测
 * Then raw200且每次均包含compute与evaluate的AdaptiveEntry，证明快速档阴性断言不是失效探针。
 */
#[test]
fn ordinary_and_strict_entries_are_observable() {
    math_inner_test_inner::ordinary_and_strict_entries_are_observable();
}

/** 契约：票据13授权接缝reset_math_trace/take_math_trace（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API无法读取或重置测试私有观测缓冲。
 * @edgecase
 * Scenario: trace_read_and_reset_clear_events
 * Given 当前线程清空后执行D3 raw1的fast_exp
 * When take后再次take，另一次调用后reset再take
 * Then 首次非空、重复读取为空、reset后为空。
 */
#[test]
fn trace_read_and_reset_clear_events() {
    math_inner_test_inner::trace_read_and_reset_clear_events();
}

/** 契约：票据13授权接缝线程隔离（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API无法观察不同线程的测试私有缓冲。
 * @edgecase
 * Scenario: trace_buffers_are_thread_isolated
 * Given 主线程清空，两个新线程起始缓冲为空，各执行D3 raw1的fast_exp并同步等待
 * When 主线程读取、一个子线程reset，另一个子线程take
 * Then 主线程仍为空，reset线程为空，另一个线程保留非空128/ToEven事件，各线程重复读取为空。
 */
#[test]
fn trace_buffers_are_thread_isolated() {
    math_inner_test_inner::trace_buffers_are_thread_isolated();
}

/** 契约：票据13授权观测接缝、固定128 bit快速近似档（无API ID）
 * @api:src/lib.rs
 * @impl:src/math_inner.rs
 * @test:src/math_inner_test.rs
 * @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md
 * @kind: unit
 * @seam: inner
 * 公共API不能观察必要后端调用点是否漏记。
 * @critical
 * Scenario Outline: required_backend_operations_are_observed
 * Given 两类D3的七方法非特例输入；另有两类D10 raw20及有符号raw-20
 * When 分别执行七方法，以及整数exp2(2)、exp2(-2)
 * Then 七方法均记录输入ratio.div、对应函数、最终scale.mul及quantize.half.div；log额外记录第二次ratio.div、log.base.ln和log.div，负幂记录powi及power.reciprocal；整数exp2记录exp2.powi，负指数额外记录power.reciprocal；所有记录为128/ToEven且无AdaptiveEntry。
 */
#[test]
fn required_backend_operations_are_observed() {
    math_inner_test_inner::required_backend_operations_are_observed();
}
