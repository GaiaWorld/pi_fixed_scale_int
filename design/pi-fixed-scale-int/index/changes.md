# 登记分册：变更日志

## 变更日志

| 日期 | 阶段 | 变更 | 影响 |
|---|---|---|---|
| | | 无（尚无回溯与提问记录） | 无 |
| 2026-10-17 | L1 | 本轮提问 12 次；需求与流程经用户确认 | L1 分册 |
| 2026-10-17 | L1 返工 | 用户将任意正分母改为正 u32；撤销朴素中间超界规则；本轮提问 4 次 | 01-requirements/ 与后续 L7 草案 |
| 2026-10-17 | L7 | 精简档；本轮提问 3 次；调用链及声明壳芯经用户确认，cargo check 通过 | 07-design/call-chain.md、src/lib.rs、src/fixed_inner.rs |
| 2026-10-17 | L1 返工 | L9 不放行后回溯补乘除舍入/饱和验收与输入换分母失败路径；本轮提问 2 次 | 01-requirements/ 改写，07-design/ 与 src/ 壳芯待 L7 重审 |
| 2026-10-17 | L7 返工 | 精简档重审；本轮提问 3 次；壳契约和示意芯伪码补齐；cargo check 和四项零分母编译实验通过 | 07-design/call-chain.md、src/lib.rs、src/fixed_inner.rs、00-context.md |
| 2026-10-17 | L1 返工 | 复审后修正直接读取及截断/舍入分支流程；本轮提问 1 次 | 01-requirements/flow.md；L7 调用链及壳芯待重审 |
| 2026-10-17 | L7 返工 | 复审补完全部验收切片及 u64 减法下溢分支；本轮提问 3 次；cargo check 和四项编译期拒绝复核通过 | 07-design/call-chain.md、src/fixed_inner.rs |
| 2026-10-17 | L1 返工 | 终审后补无符号减法下溢图分支；本轮提问 1 次 | 01-requirements/flow.md；L7 切片回溯核对 |
| 2026-10-17 | L7 返工 | 调用链和切片复用已确认冻结壳芯；本轮提问 2 次 | 07-design/call-chain.md 与 src/ 契约对拍 |
