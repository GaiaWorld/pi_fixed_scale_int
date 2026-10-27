# pi-fixed-scale-int · 冻结调用链与实现切片

## 调用链

```text
调用方
  → src/lib.rs：固定公开类型、方法、分母使用约束
    → src/fixed_inner.rs：整数算术 / 舍入 / rescale / 十进制
    → src/math_inner.rs（math feature）：三档数学方法
      → astro-float：高精度后端
    → serde / fory（各自 feature）：raw 编解码
  ← 原类型与分母的结果，或明确错误

数值 = raw / D；成功和失败均不修改源操作数
```

上图是职责边界，不要求固定私有函数名。公开方法声明中的 todo! 需由受测者完成。

## 实现切片与公开验收

| 切片 | 行为 | 主要测试 |
|---|---|---|
| 1 | 常量、两类数值、正分母、raw 边界 | tests/ticket_1_test.rs |
| 2 | 整数构造与比较、跨分母加减拒绝 | tests/ticket_2_test.rs |
| 3 | checked / saturating 加减 | tests/ticket_3_test.rs |
| 4 | checked / saturating 乘除、除零 | tests/ticket_4_test.rs |
| 5 | 半值远离零乘除与范围边界 | tests/ticket_5_test.rs |
| 6 | rescale、整数输出、零目标分母 | tests/ticket_6_test.rs |
| 7 | 十进制、浮点最短文本、Display、Debug | tests/ticket_7_test.rs、tests/decimal_api.rs |
| 8 | 混合乘除、原生整数、serde/fory | tests/ticket_8_test.rs、tests/serialization.rs |
| 9 | 普通数学、误差、定义域与范围 | tests/ticket_9_test.rs 及 additional / power_edges |
| 10 | strict 数学、独立有理数基准、边界 | tests/ticket_10_test.rs 及 additional |
| 11 | 数学中间失败、范围与负幂保护 | tests/ticket_11_test.rs |
| 12 | 近半值、独立整数基准 | tests/ticket_12_test.rs、tests/math.rs |
| 13 | 七个 fast 方法、类型/分母/定义域 | tests/ticket_13_test.rs |
| 14 | fast 独立参考、错误状态、其他档回归 | tests/ticket_14_test.rs |

切片是组织建议，可以自主选择实现顺序。测试里的 support 文件包含实际断言，必须与声明文件一起保留。

## 基础数值义务

- 原生整数输入乘 D；from_raw/raw 不缩放。
- 浮点按原始 f32/f64 的 to_string 最短十进制文本转换；f32 不先升为 f64。
- 乘法目标 raw = lhs_raw * rhs_raw / rhs_D；除法目标 raw = lhs_raw * rhs_D / rhs_raw。
- checked 向零截断；rounded 半值远离零；saturating 截断后夹边，但除零仍返回错误。
- 先量化再按最终 raw 判断范围，无符号微小负数可以量化到零。
- Display 最多 18 位小数，半值远离零，删除末尾零，不接受格式器 precision/width 改写该规则。
- D=0 的公开数值使用处和 TO=0 的 rescale 使用处编译失败；单纯提及类型、Debug 与派生序列化不作为分母验证入口。

## 普通 / strict 的证明义务

1. raw 与 D 在起始精度精确表示，输入区间必须封闭真实有理数。
2. 指数、对数等组合使用合法定义域和单调性构造向外舍入界。
3. 整数负幂安全处理 i32::MIN，倒数前验证中间幂；零次幂为 1，只缩放一次。
4. 指定底数对数按 ln(x)/ln(base) 组合；分母界含零则提升精度，不能直接除。
5. 除法四角先逐个检查有效性，再选上下界，不能用 min/max 吞掉 NaN 或底层错误。
6. 最终区间单位为 raw。strict 仅在能证明正确量化时成功；普通在已证明至多 1 raw 误差时成功。
7. 范围跨界而未能证明时继续升精度；预算耗尽报 PrecisionExhausted，不猜最后一位或降级档位。
8. 独立数学界可以用于零/越界早退；没有证明时不把无穷取倒数当成精确零。

## fast 合同

两类数值均提供 fast_exp、fast_exp2、fast_ln、fast_log2、fast_log10、fast_log(base: Self)、fast_powi(n: i32)。

每个后端算术/函数请求为 128 bit、ToEven 的单值计算，包括最终乘 D；最后对近似 raw 使用半值远离零量化。禁止自适应、上下界及回退；不承诺统一 1 raw 误差或固定倍数加速，不返回 PrecisionExhausted。

对数定义域、0 的负幂、近似范围、后端异常分别遵守方法注释。普通/strict 与 fast 不能靠互相比较替代独立正确性验收。

## 验收边界

旧实现依赖私有 trace、测试区间注入、可控预算的单元测试不包含在本起点。公开数值测试不能完整观测内部请求精度和算法路径；这些合同须通过实现审查或评测维护者独立插桩验收，不能仅凭公开测试通过宣称已证明。
