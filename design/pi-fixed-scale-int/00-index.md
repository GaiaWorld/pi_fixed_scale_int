# pi-fixed-scale-int · 冻结设计索引

状态：设计、公开接口和公开模块测试已冻结；全部业务方法待实现。

| 分册 | 用途 |
|---|---|
| 00-context.md | 评测起点与模块范围 |
| 01-requirements/needs.md | 完整数值需求及验收规则 |
| 01-requirements/problem.md | 业务问题与范围 |
| 01-requirements/notes.md | 舍入、输入与失败语义澄清 |
| 01-requirements/flow.md | 基础业务流程 |
| 05-modules/modules.md | 模块职责与边界 |
| 05-modules/deps.md | feature 与依赖关系 |
| 05-modules/decisions.md | 冻结的数学精度与依赖选择 |
| 05-modules/change-points.md | 实现影响范围 |
| 07-design/call-chain.md | 调用链、实现切片与证明义务 |
| index/glossary.md | 术语 |
| index/status.md | 当前状态 |
| index/changes.md | 本起点变更记录 |

具体公开签名和逐方法契约以 src/lib.rs、src/math_inner.rs 为准；输入显示的细节见 docs/usage.md。发生实质矛盾时由评测维护者裁定，不让受测者通过修改测试消除冲突。
