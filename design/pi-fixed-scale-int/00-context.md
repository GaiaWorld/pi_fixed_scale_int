# pi-fixed-scale-int · 评测设计上下文

本分支是设计与接口已确定、生产行为尚未实现的评测起点。题目是按本设计实现整个库，而非重新设计公开 API。

- 数值模型为 raw / D，raw 使用 u64 或 i64，D 为正 u32 常量。
- Rust 与依赖分别由 rust-toolchain.toml、Cargo.lock 固定；默认功能无外部运行时依赖。
- src/lib.rs 冻结公开类型与基础方法签名；src/math_inner.rs 冻结 math 方法与 MathError。
- src/fixed_inner.rs 保留模块职责，私有函数由受测者组织。接口方法内的 todo! 是预期未实现状态。
- tests/ 是公开、不可修改的模块契约测试，不是隐藏评分集。其通过不构成数学全域证明。
- 原实现的私有 trace、区间注入和预算接缝测试不纳入本起点，避免绑定旧算法组织。
- 评测操作、限制与验证方法见根目录 BENCHMARK.md。
