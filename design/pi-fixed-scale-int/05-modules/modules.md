# pi-fixed-scale-int · 冻结模块边界

| 模块 | 职责 |
|---|---|
| src/lib.rs | 两类数值、常量、错误、FixedOperand、转换、比较和基础方法入口 |
| src/fixed_inner.rs | 精确整数算术、舍入、范围检查、比例和十进制转换的私有实现 |
| src/math_inner.rs | math 公开方法与 MathError；普通、strict、fast 数学计算 |
| serde / fory | 按 feature 派生 raw 编解码 |
| tests/ | 从公开 API 观察的模块契约测试，不约束私有函数名 |

底层存储保持 64 位；数学内部高精度不能扩大公开存储范围。成功和失败均不修改输入值与外部状态。

公开层的行为体可以修改；不改变方法签名或通过更改测试、依赖、契约来降低题目要求。可以在 src/ 添加合理的私有实现文件。
