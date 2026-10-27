# pi-fixed-scale-int · 冻结设计决定

## 数值与依赖

- 基础计算使用宽整数完成缩放、舍入及范围检查，不以 f64 近似替代精确十进制或 64 位整数运算。
- math 使用纯 Rust astro-float 0.9.6（实际解析见 Cargo.lock），默认关闭；serde、fory 独立可选。
- serde/fory 保存 raw 字段，D 由类型决定；不要求两个协议字节互通。
- 评测工具链为 Rust 1.94.1；本机验证目标为 x86_64-pc-windows-msvc，不混用 32 位或其他链接环境。

## 数学三档

| 档位 | 后端计算 | 成功保证 | 不能确认时 |
|---|---|---|---|
| 普通 | 128 bit 起，每次翻倍，上限 12288 bit | 距正确半值远离零舍入至多 1 raw | PrecisionExhausted |
| strict | 192 bit 起，每次翻倍，上限 12288 bit | 可证明的正确半值远离零舍入 | PrecisionExhausted |
| fast | 固定请求 128 bit、ToEven 的单值链路 | 最终近似 raw 按半值远离零量化；无统一数学误差界 | Domain / Overflow / ComputationFailed，不返回 PrecisionExhausted |

普通/strict 使用有向区间封闭真值；fast 不建立上下界、不自适应、不回退。后端内部保护位不受请求位数限制，固定请求精度不代表固定运行时间。

有限、无后端错误且顺序正确的端点才能参与区间组合。四角除法聚合前逐项检查异常，倒数前确认正区间；无独立数学界证明的无穷不能当作可用有限端点。

依赖的公开正确舍入合同和实际 Up/Down 方向应由实现者核查，入口：https://docs.rs/astro-float/0.9.6/astro_float/ 。正确组合的证明义务见 ../07-design/call-chain.md。

## 模块自由度

公开签名、数值类型、错误枚举、feature 和依赖冻结；模块内私有结构、辅助函数及复用方式可以调整。禁止以另一版同名库或原实现代替待实现逻辑。
