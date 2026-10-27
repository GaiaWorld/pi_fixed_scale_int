# pi-fixed-scale-int 设计索引（design-lite）

恢复入口：index/status.md 的当前阶段。

## 登记分册

| 分册 | 内容 |
|---|---|
| index/status.md | 模式与阶段 |
| index/glossary.md | 术语 |
| index/changes.md | 提问及变更 |

## 内容分册

| 分册 | 内容 | 阶段 |
|---|---|---|
| 00-context.md | 地形、来源 | L0 |
| 01-requirements/problem.md | 问题与范围 | L1 |
| 01-requirements/needs.md | 需求与验收 | L1 |
| 01-requirements/flow.md | 流程图 | L1 |
| 01-requirements/notes.md | 非功能与澄清 | L1 |
| 05-modules/modules.md | 本次触达的模块边界 | L5 |
| 05-modules/deps.md | 模块依赖边与依赖图 | L5 |
| 05-modules/change-points.md | 变更点与影响面 | L5 |
| 05-modules/decisions.md | 数学依赖选型与可行性 | L5 |
| 07-design/call-chain.md | 已有切片 1–6 与新增切片 7–10 的调用链/验收 | L7 |

本轮为既有设计返工：原审查报告已归档，新 L9 审查待完成。现有生产代码已实现新增行为，切片 7–10 记为“已落地、待设计复核”，并不表示数学全域精度已获审查放行。
