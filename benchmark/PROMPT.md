# 任务：实现固定分母的64位数值库

请一次自主完成本仓库 pi_fixed_scale_int 的全部待实现行为。设计、公开接口和公开模块契约测试已经给出，无需重新设计公共 API。

阅读顺序：
1. BENCHMARK.md 的范围与限制。
2. design/pi-fixed-scale-int/00-index.md、01-requirements/needs.md。
3. docs/usage.md 以及 src/lib.rs、src/math_inner.rs 的逐方法契约。
4. 07-design/call-chain.md 的模块切片与证明义务，tests/ 及 support/ 的公开测试。

实现范围包括两类 raw/D 数值、任意正u32分母、转换/比较、checked/rounded/saturating算术、混合乘除、rescale、十进制解析与显示、math普通/strict/fast、serde/fory兼容。给定派生和分母断言是脚手架，其余 todo! 均需实现。

只修改 src/ 的实现，必要时新增私有实现文件；额外自测放 src/ 独立测试文件或 extra-tests/。不要修改公开测试、设计、依赖、工具链和评测文件，不删除功能或用样例硬编码，不读取原库、历史、其他模型结果或参考实现。

可以自主运行公开测试并修复。优先保证行为正确、算法可解释、边界与错误完整，代码保持适度复用与可读性。严格数学成功必须有舍入证明，不能猜最后一位；fast遵守固定128bit请求预算且不回退。

完成后概述实现、执行过的验证及仍未通过的项目，不把未经运行的测试报告为通过。运行时间、token及评分由外部评测器记录。
