# pi_fixed_scale_int

> 当前分支是实现评测种子：设计、公开接口和模块测试已冻结，业务方法为 `todo!()`。下文示例是待实现的目标行为。任务入口与使用方法见 [BENCHMARK.md](BENCHMARK.md)。

固定分母的 64 位整数运算库。用整数 raw 保存计数，数值为 **raw / D**；分母 D 是类型中的正 u32 常量，可以是 100、3 或其他非零值。

- `FixedU64<D>`：raw 为 u64；`FixedI64<D>`：raw 为 i64。
- 提供十进制文本、原始整数和浮点输入，以及十进制显示。
- 同类同分母加减、比较；乘除可接受不同符号类别、不同分母或受支持的原始整数，结果保持左值类型和分母。
- 明确区分向零截断、半值远离零舍入和饱和运算；改变分母必须显式 rescale。
- 可选 math、serde、fory 功能，默认均不开启。

## 安装

在使用方的 Cargo.toml 中添加（对应本仓库清单版本 0.2.0）：


```toml
[dependencies]
pi_fixed_scale_int = "0.2.0"

# 按需开启，可组合：
# pi_fixed_scale_int = { version = "0.2.0", features = ["math", "serde", "fory"] }
```

本地使用也可写成 `pi_fixed_scale_int = { path = "../pi_fixed_scale_int" }`。

本项目使用 Rust 2024 edition，需要支持该 edition 的工具链（Rust 1.85 起）；清单未声明 rust-version，启用可选依赖后还需满足依赖自己的工具链和目标平台要求。详见[安装与 feature](docs/usage.md#安装与-feature)。

## 快速开始


```rust
use pi_fixed_scale_int::{FixedI64, FixedU64};

fn main() {
    type Amount = FixedI64<100>;
    let amount: Amount = "1.25".parse().unwrap();
    assert_eq!(amount.raw(), 125);
    assert_eq!(amount.to_string(), "1.25");

    let total = amount.checked_add(Amount::try_from_integer(2).unwrap()).unwrap();
    assert_eq!(total.raw(), 325); // 3.25

    let half = FixedU64::<10>::from_raw(5); // 0.5，不同类别、不同分母
    assert_eq!(amount.rounded_mul(half).unwrap().raw(), 63); // 0.625 -> 0.63
    assert_eq!(amount.checked_mul(2u8).unwrap().raw(), 250);

    let tenths = amount.rescale_rounded::<10>().unwrap();
    assert_eq!(tenths.raw(), 13); // 1.25 -> 1.3
    assert_eq!(total.to_integer(), 3);
}
```

注意：`from_raw(125)` 在 D=100 时表示 1.25；`try_from_integer(125)` 表示完整整数 125，需要 raw=12_500。

## 可选数学函数

启用 `features = ["math"]` 后，两类数值均提供指数、对数和整数次幂：


```rust
use pi_fixed_scale_int::FixedI64;

fn main() {
    type Number = FixedI64<1_000_000>;
    let one = Number::try_from_integer(1).unwrap();
    assert_eq!(one.exp_strict().unwrap().raw(), 2_718_282);
    let two = Number::try_from_integer(2).unwrap();
    assert_eq!(two.powi_strict(-3).unwrap().raw(), 125_000);
}
```

三档的成功保证不同：

| 档位 | 例子 | 行为 |
| --- | --- | --- |
| fast | fast_exp() | 后端请求固定 128 bit，单值近似，无统一数学误差界，不自适应升精度 |
| 普通 | exp() | 128 bit 起自适应区间计算；成功结果与正确舍入 raw 相差至多 1 |
| 严格 | exp_strict() | 192 bit 起自适应区间计算；成功结果是半值远离零的正确舍入值 |

普通和严格档的精度预算上限为 12_288 bit；不能完成证明时会报 PrecisionExhausted。fast 不返回该错误，但可能报 ComputationFailed，且不承诺固定耗时或性能倍数。严格档并不保证每次都成功。

## 文档与验证

[完整使用说明](docs/usage.md)涵盖全部方法、常量、输入规则、范围与舍入边界、序列化格式及数学错误语义。

在已具备对应目标编译与链接环境的仓库根目录运行：


```sh
cargo test
cargo test --features math
cargo test --features serde
cargo test --doc --features math
```

部分集成测试会直接启动 PATH 中的 rustc 并生成临时程序，因此还需要可用的编译器、链接器及临时目录权限。fory 及组合 feature 的命令见[验证命令](docs/usage.md#验证命令)。
