# 固定分母数值库：完整实现评测起点

## 固定起点

本分支 benchmark/implementation-seed 提供已冻结的设计、公开接口与公开模块契约测试，生产行为留为 todo!。这是“有设计指导的完整实现题”，不是无约束从零设计题。

已提供类型、常量、Clone/Copy、feature 序列化派生和分母常量断言；这些不属于本题待实现工作。其他函数体（包括 lib.rs 内转换/比较、错误 Display）均待实现。数学接口保留在 src/math_inner.rs。私有函数和辅助文件可以自行组织。

受测者入口：benchmark/PROMPT.md。需求与模块索引：design/pi-fixed-scale-int/00-index.md。完整数值细节：docs/usage.md。

## 正式测量必须导出干净副本

这个维护仓库的其他分支及 Git 历史包含原实现，target 也可能有旧构建产物。不要直接在维护仓库中启动受测模型。

由评测维护者在本仓库运行（目标目录必须不存在，父目录需存在）：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .enchmarkexport-seed.ps1 -Destination "C:UsersAdministratorAppDataLocalTempopencodeixed-scale-model-a-run1"
```

脚本仅导出指定评测分支的已提交文件，创建无原提交历史、无 remote 的独立 Git 工作区；不会复制维护仓库 .git、target 或忽略文件。输出包括精确源提交与归档 SHA256，由维护者在运行记录中保存。

正式执行时还应限制模型的文件系统/网络访问，避免读取原库或其他样本；导出不是安全沙箱。相同模型/框架的各次运行使用不同导出目录，不覆盖样本。

## 允许与禁止

- 允许修改 src/ 中的实现体、组织私有函数和新增合理的私有实现文件。
- 允许在 src/ 添加独立测试文件或在 extra-tests/ 添加自测；不要改动已提供的 tests/。
- 保持公开签名、常量、错误枚举、值类型表示、feature、依赖和既定语义不变。
- 不得修改 tests/、设计/使用文档、Cargo.toml、Cargo.lock、rust-toolchain.toml 或评测文件来规避要求。
- 不引入原版 pi_fixed_scale_int 作为依赖，不读取维护库历史或复制参考实现，不按测试样例硬编码。
- 一次任务允许自主读代码、编译、测试、修复；不提供人工中途提示。

## 工具链与测试

Rust 固定 1.94.1，依赖由 Cargo.lock 固定。本机参考验证目标是 x86_64-pc-windows-msvc，需要正确的链接环境；其他平台必须另做预检。

准备阶段由维护者执行 cargo fetch --locked，再离线预检。默认、math、serde、fory 及全部组合应分别验证：

```powershell
cargo check --locked --offline --all-features --all-targets
powershell -NoProfile -ExecutionPolicy Bypass -File .enchmark	est-matrix.ps1 -BuildOnly
powershell -NoProfile -ExecutionPolicy Bypass -File .enchmark	est-matrix.ps1
```

BuildOnly 只确认各配置可编译；正常模式运行公开测试并保存每个 feature 配置的日志和 summary.json。默认输出 .benchmark-results/，可指定 -OutputDirectory。占位起点的运行时测试应失败，脚本退出非零是预期；不能把编译错误、零测试或环境错误当作有效的起点失败。

tests/ 含 14 个切片及补充测试，support/ 是断言实现，必须一起保留。fast 零分母测试不绑定私有函数名。源实现专用的 26 个私有接缝测试未纳入本题；普通/strict 的精度证明、fast 固定请求精度等内部约束仍须代码审查，不因移除测试而取消合同。

## 记录与评分边界

本次只建立公开实现题起点；现有测试都是公开测试，不冒充独立隐藏评分集。benchmark/frozen-files.json 提供冻结输入的 SHA256 清单（仅归一 CRLF 为 LF，保留 BOM），测试入口会先核对；seed.json 标识初始状态。

正式运行前在所有参赛配置间统一并记录：模型和参数、框架版本、题目提交、提示词、时间/token 上限、缓存/构建策略、并发度和环境。工具链和依赖预下载不计模型实现时间；模型期间的构建、测试、修复计入总耗时。不要复用原实现或其他参赛者的 target。

评分记录至少包含：按模块的准确率、完整通过标记、独立代码质量审查、总耗时及模型/工具/构建拆分、输入/缓存/输出/推理 token、调用次数与费用。跨模型 token 不同 tokenizer，不直接等价于费用。

建议权重准确性60/质量20/时间10/token10，尚未冻结为正式排名规则；需校准后在首轮排名前确定。效率分应受准确性约束，避免快速交付空实现得高分。隐藏测试与最终评分运行器留在模型工作区之外。
