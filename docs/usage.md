# 使用说明

[返回 README](../README.md)

本文定义本评测分支的目标行为，不表示占位实现已经完成。下文用 raw 表示底层计数，D 表示源分母，TO 表示目标分母。

## 安装与 feature

```toml
[dependencies]
pi_fixed_scale_int = "0.2.0"
# 或启用所需组合：
# pi_fixed_scale_int = { version = "0.2.0", features = ["math", "serde", "fory"] }
```

本地依赖可使用 path。以上 version 对应当前 Cargo.toml，不代表对外部注册表可用性的检查结果。

| feature | 默认开启 | 作用及依赖 |
| --- | --- | --- |
| math | 否 | 数学方法与 MathError；astro-float 0.9.6，关闭其默认 feature，开启 std |
| serde | 否 | 派生 Serialize/Deserialize；serde 1，开启 derive |
| fory | 否 | 派生 ForyStruct；fory 1.7.7 |

这些 feature 互不依赖，可以组合；没有名为 default 的默认功能集合。清单中的依赖版本是 Cargo 版本约束，最终解析版本由使用方的解析结果和锁文件决定。使用方要直接调用 serde_json 或 fory 时，应在自己的 Cargo.toml 中声明相应依赖；本库的 serde feature 不会为使用方直接导出 serde_json。

代码使用 std，并非 no_std 库。Rust 2024 edition 本身要求 Rust 1.85 或更新工具链，但本库没有声明 rust-version 或经过验证的最低支持版本；可选依赖可能有更高要求。目标平台还须满足依赖和链接环境要求，不承诺所有 feature 在所有平台无条件可用。Windows 环境也需安装与 Rust target 匹配的链接工具链，不能把 MSVC 与 GNU 环境混为一谈。

## 数值模型与分母约束

```text
数值 = raw / D
最小步长 = 1 / D
```

| 类型 | raw 类型与范围 | 数值范围（精确有理数） |
| --- | --- | --- |
| FixedU64<D> | u64，0 至 18_446_744_073_709_551_615 | 0 至 u64::MAX / D |
| FixedI64<D> | i64，-9_223_372_036_854_775_808 至 9_223_372_036_854_775_807 | i64::MIN / D 至 i64::MAX / D |

两类均实现 Clone、Copy，raw 字段私有；方法按值接收，不修改原对象。D 的类型是 u32，正常使用范围为 1..=u32::MAX；不是小数位数，也不要求为 10 的幂。分母越大，步长越细，能表示的完整整数范围越小。

构造、读取 raw、解析、显示、算术、数学及比较等数值操作内的常量断言会在实例化 D=0 时触发编译错误；rescale 的 TO=0 同理。这不是运行时 Result 错误。不要把它理解为类型名称本身不能出现：单纯提及 FixedI64<0> 不一定触发断言；Debug 和派生序列化也不是分母验证入口，反序列化不会调用 from_raw 检查 D。应用协议必须坚持使用正分母。

### 便捷分母常量

以下均为根模块导出的 u32 常量，可用于 const 泛型；没有预设数值类型别名或 ONE/MAX 关联常量。

| 常量 | 值 |
| --- | ---: |
| SCALE_10 | 10 |
| SCALE_100 | 100 |
| SCALE_1000 | 1_000 |
| SCALE_10000 | 10_000 |
| SCALE_100000 | 100_000 |
| SCALE_1000000 | 1_000_000 |
| SCALE_10000000 | 10_000_000 |
| SCALE_100000000 | 100_000_000 |
| SCALE_75600 | 75_600 |
| SCALE_324000 | 324_000 |
| SCALE_1371600 | 1_371_600 |
| SCALE_2268000 | 2_268_000 |
| SCALE_9601200 | 9_601_200 |

```rust
use pi_fixed_scale_int::{FixedI64, SCALE_100};
type Amount = FixedI64<SCALE_100>;
assert_eq!(Amount::from_raw(123).to_string(), "1.23");
```

## 构造与输入

### raw 与完整整数

下表中底层整数类型 T 对 FixedU64 是 u64，对 FixedI64 是 i64。

| 入口 | 语义 | 返回值 |
| --- | --- | --- |
| from_raw(raw: T) | 原样存储 raw，不乘 D | Self |
| raw(self) | 原样取出 raw | T |
| try_from_integer(value: T) | 将完整整数乘 D，再检查 raw 范围 | Result<Self, ArithmeticError> |
| try_from_number<I: Into<i128>>(value: I) | 将能转为 i128 的整数乘 D，检查中间值和目标范围 | Result<Self, ArithmeticError> |
| TryFrom<原始整数> | 完整整数转换，不是 raw 转换 | Result<Self, ArithmeticError> |

TryFrom 支持 u8/u16/u32/u64/u128/usize 和 i8/i16/i32/i64/i128/isize。大于 i128::MAX 的 u128 会返回 Overflow。try_from_number 的参数受 Into<i128> 约束，不等同于所有这些 TryFrom 实现；如需 u128、usize、isize 输入，可使用 TryFrom。任何最终 raw 越界或无符号目标无法表示的负整数均返回 Overflow。

```rust
use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64};
type N = FixedI64<100>;
assert_eq!(N::from_raw(125).raw(), 125);            // 数值 1.25
assert_eq!(N::try_from_integer(125).unwrap().raw(), 12_500); // 数值 125
assert_eq!(N::try_from_number(2u8).unwrap().raw(), 200);
assert_eq!(N::try_from(2u128).unwrap().raw(), 200);
assert_eq!(FixedU64::<100>::try_from(-1i8), Err(ArithmeticError::Overflow));
```

### 十进制文本

实现 FromStr，可调用字符串的 parse 或类型的 from_str。输入先以十进制精确计算乘 D，再按最近值舍入，恰好半值时远离零，最后检查目标 raw 范围；不经过 f64。

- 支持 ASCII 数字、开头的 + / -、至多一个小数点以及 e/E 科学计数法。
- 至少要有一个数字；.5、1.、+1.25、1e-2 均可解析。
- 指数按 i32 解析；非法或超出 i32 的指数归为 Invalid。
- 不自动 trim；空白、下划线、千位逗号、NaN、inf 等不是合法数值文本。
- 无符号目标也先舍入再检查范围：D=10 时 -0.01 舍入成 raw=0 可以成功，-0.05 舍入成 raw=-1 则 Overflow。

```rust
use pi_fixed_scale_int::{FixedI64, FixedU64, ParseFixedError};
assert_eq!("-1.25".parse::<FixedI64<10>>().unwrap().raw(), -13);
assert_eq!("1.235".parse::<FixedU64<100>>().unwrap().raw(), 124);
assert_eq!("1.25e2".parse::<FixedU64<100>>().unwrap().raw(), 12_500);
assert_eq!("-0.01".parse::<FixedU64<10>>().unwrap().raw(), 0);
assert_eq!(" 1".parse::<FixedI64<10>>(), Err(ParseFixedError::Invalid));
```

### 浮点输入

try_from_f32(f32)、try_from_f64(f64) 和对应 TryFrom 均返回 Result<Self, ArithmeticError>。

它们先检查有限性，再使用**原始浮点类型自身的 value.to_string() 十进制文本**进行上述解析和舍入。f32 不会先拓宽为 f64。规则不是对 IEEE 二进制精确值乘 D；接近半值时，这一区别很重要。原始浮点数已有的信息损失不会被恢复。

NaN、正负无穷、解析转换失败或舍入后 raw 越界统一返回 ArithmeticError::Overflow。微小数可舍入成零，无符号目标的微小负数也适用先舍入后检查规则。

```rust
use pi_fixed_scale_int::{ArithmeticError, FixedI64};
assert_eq!(FixedI64::<10>::try_from_f32(-1.25).unwrap().raw(), -13);
assert_eq!(FixedI64::<10>::try_from(1.25f64).unwrap().raw(), 13);
assert_eq!(FixedI64::<10>::try_from_f64(f64::NAN), Err(ArithmeticError::Overflow));
```

## Display 与 Debug

Display 输出 raw / D 的普通十进制数，不用科学计数法：

1. 以整数长除法生成小数，余数为零时结束；否则最多生成 **18 位小数**。
2. 仍有余数时，在第 18 位按最近值舍入，半值远离零，并处理向整数部分的进位。
3. 删除小数末尾的零；没有小数部分时删除小数点；零输出为 0。

因此循环小数不是无限精确输出；即使是终止小数，所需位数超过 18 也会舍入。当前实现不使用 Formatter 的 precision/width 等选项，不能用 format!("{:.2}", value) 指定两位小数。需要固定小数位的业务格式应自行处理。

```rust
use pi_fixed_scale_int::{FixedI64, FixedU64};
assert_eq!(FixedU64::<3>::from_raw(1).to_string(), "0.333333333333333333");
assert_eq!(FixedU64::<3>::from_raw(2).to_string(), "0.666666666666666667");
assert_eq!(FixedI64::<100>::from_raw(-120).to_string(), "-1.2");
assert_eq!(format!("{:?}", FixedI64::<10>::from_raw(-12)),
           "FixedI64 { raw: -12, denominator: 10 }");
```

Debug 当前输出类型名、raw 和 denominator，适合诊断；它不是解析或序列化协议。

## 算术、比较与舍入

本库通过具名方法提供算术，没有实现 +、-、*、/ 等运算符。输入为 Copy，成功或失败都不修改输入。

### 加减与比较

checked_add(self, rhs: Self)、checked_sub(self, rhs: Self) 返回 Result<Self, ArithmeticError>。saturating_add、saturating_sub 返回 Self，越界时夹到相应 raw 上下限；无符号下限为零。

加减与 PartialEq/Eq/PartialOrd/Ord 比较均要求**同类别、同分母**。FixedI64<100> 与 FixedI64<10> 或 FixedU64<100> 不能直接比较或相加。比较相当于比较 raw；需要不同分母参与时先显式统一分母，同时考虑重缩放舍入。

### 乘除与 FixedOperand

所有乘除方法接收 R: FixedOperand。库内实现包括：

- 任意正分母的 FixedU64<D>、FixedI64<D>，按值传入；
- u8/u16/u32/u64、i8/i16/i32/i64，这些整数的分母视为 1。

没有为 u128/i128/usize/isize 或浮点数实现 FixedOperand；它的整数支持集合与构造用的 TryFrom 集合不同。trait 的公开签名是 parts(self) -> (i128, u32)。自定义实现必须遵守正分母约定，不能把后端看作支持任意 i128 大小乘积的通用有理数引擎：内置安全中间量建立在 64 位计数范围上。非法自定义分母或超大中间量可能返回 Overflow，saturating_mul 的内部 expect 还可能 panic。

设左值 raw=a、分母 D，右值 parts()=(b, E)：

```text
乘法目标 raw = a * b / E
除法目标 raw = a * E / b
结果类型 = 左值类型，结果分母 = D
```

| 方法 | 舍入与范围处理 | 返回值 |
| --- | --- | --- |
| checked_mul / checked_mul_with | 向零截断，然后检查 raw 范围 | Result<Self, ArithmeticError> |
| checked_div / checked_div_with | 向零截断，然后检查 raw 范围 | Result<Self, ArithmeticError> |
| rounded_mul / rounded_mul_with | 最近值，半值远离零，然后检查 raw 范围 | Result<Self, ArithmeticError> |
| rounded_div / rounded_div_with | 最近值，半值远离零，然后检查 raw 范围 | Result<Self, ArithmeticError> |
| saturating_mul / saturating_mul_with | 向零截断，然后夹到 raw 上下限 | Self |
| saturating_div / saturating_div_with | 向零截断，然后夹到 raw 上下限 | Result<Self, ArithmeticError> |

每对带 / 不带 _with 的方法行为相同。饱和乘除不是 rounded 乘除；没有同时“半值远离零并饱和”的公开入口。除法的所有形式遇到 b=0 都返回 DivisionByZero，饱和除法也不会把除零变成最大值。判定的是右值本身的零 raw，不是其 to_integer() 是否为零。

向零截断不同于向负无穷取整：-0.5 raw 截断到 0；rounded 则舍入到 -1。无符号左值参与负数乘除时先量化，若结果为 0 可成功，仍为负则 checked/rounded 返回 Overflow，饱和方法夹到 0。

```rust
use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64};
let x = FixedI64::<100>::from_raw(125); // 1.25
let half = FixedU64::<10>::from_raw(5);
assert_eq!(x.checked_mul(half).unwrap().raw(), 62); // 截断 62.5
assert_eq!(x.rounded_mul_with(half).unwrap().raw(), 63);
assert_eq!(x.checked_div(half).unwrap().raw(), 250);
assert_eq!(x.checked_mul(-2i8).unwrap().raw(), -250);
assert_eq!(x.saturating_div(0u8), Err(ArithmeticError::DivisionByZero));
assert_eq!(FixedU64::<1>::from_raw(u64::MAX).saturating_mul(2u8).raw(), u64::MAX);
```

## 显式重缩放与整数输出

| 方法 | 计算 | 返回值 |
| --- | --- | --- |
| rescale::<TO>() | raw * TO / D，向零截断 | Result<同类别<TO>, ArithmeticError> |
| rescale_rounded::<TO>() | raw * TO / D，半值远离零 | Result<同类别<TO>, ArithmeticError> |
| to_integer() | raw / D，向零截断 | u64 或 i64 |
| to_integer_rounded() | raw / D，半值远离零 | u64 或 i64 |

重缩放不改变符号类别。增大分母可能使目标 raw 越界并返回 Overflow；减小分母可能丢失精度。没有隐式重缩放或饱和重缩放入口。整数输出无需 Result；返回类型与底层 raw 类型相同，但返回值已不再是 raw。

```rust
use pi_fixed_scale_int::FixedI64;
let x = FixedI64::<100>::from_raw(-125);
assert_eq!(x.rescale::<10>().unwrap().raw(), -12);
assert_eq!(x.rescale_rounded::<10>().unwrap().raw(), -13);
let y = FixedI64::<10>::from_raw(-15);
assert_eq!(y.to_integer(), -1);
assert_eq!(y.to_integer_rounded(), -2);
```

## 错误类型

| 枚举 | 变体 | 含义 |
| --- | --- | --- |
| ParseFixedError | Invalid | 文本语法非法，含非法或越界的 i32 指数 |
| ParseFixedError | Overflow | 解析中间范围或舍入后的目标 raw 超界 |
| ArithmeticError | Overflow | 输入转换、算术或重缩放无法表示；浮点非有限值也映射到此项 |
| ArithmeticError | DivisionByZero | 算术除数为零 |
| MathError（math） | Domain | 数学定义域错误 |
| MathError（math） | Overflow | 数学档位相应的量化结果超出 raw 范围，或独立数学界已证明溢出 |
| MathError（math） | PrecisionExhausted | 普通/严格档预算内无法证明要求的舍入或误差界 |
| MathError（math） | ComputationFailed | 后端错误、NaN、无独立界证明的无穷或无效中间值/区间 |

三种枚举均实现 Clone、Copy、Debug、Eq、PartialEq。ParseFixedError 和 MathError 还实现 Display 与 std::error::Error，Display 当前使用变体名；ArithmeticError 当前未实现这两个 trait。将示例改成返回通用错误并用 ? 时要考虑这一差异。

## serde 与 fory 序列化

这两项均按结构体 **raw 字段**派生，不是 Display 的十进制字符串，不是数值 raw / D，也没有额外的 denominator 数据字段。D 由具体泛型类型决定。发送方和接收方必须约定同一符号类别、分母与协议；序列化不执行 rescale。

### serde

需开启 serde feature。以 serde_json 为例：FixedI64<100> 的 raw=123 编码为对象 {"raw":123}，而不是字符串 "1.23" 或数字 1.23。其他 serde 格式按各自的结构体编码规则处理。

使用方还需依赖 serde_json = "1"：

```rust
use pi_fixed_scale_int::FixedI64;
let value = FixedI64::<100>::from_raw(123);
let json = serde_json::to_string(&value).unwrap();
assert_eq!(json, r#"{"raw":123}"#);
let decoded: FixedI64<100> = serde_json::from_str(&json).unwrap();
assert_eq!(decoded.raw(), 123);
// JSON 没有分母；相同 JSON 被读为 D=10 时，数值将是 12.3。
let other: FixedI64<10> = serde_json::from_str(&json).unwrap();
assert_eq!(other.to_string(), "12.3");
```

反序列化的字段类型为 u64/i64；超范围字段由序列化框架报错，不返回 ParseFixedError。它也不验证 D>0，不能用它绕过正分母约定。

### fory

需开启 fory feature，使用方直接操作编解码器时还需依赖 fory（本库清单约束 1.7.7）。派生的是带 raw 字段的 Fory 结构体格式，完整字节包含 Fory 自身协议结构，不能把它当成固定 8 字节裸整数。

当前集成测试使用 xlang(false)，为每个具体类型注册 ID，再进行同类型往返；这不能推出任意跨语言模式、不同 ID 配置或跨版本字节格式都兼容。

```rust
use pi_fixed_scale_int::{FixedI64, FixedU64};
let mut codec = fory::Fory::builder().xlang(false).build();
codec.register::<FixedU64<100>>(100).unwrap();
codec.register::<FixedI64<10>>(101).unwrap();
let value = FixedI64::<10>::from_raw(-12);
let bytes = codec.serialize(&value).unwrap();
let decoded: FixedI64<10> = codec.deserialize(&bytes).unwrap();
assert_eq!(decoded.raw(), -12);
```

注册不同分母的类型时须为编解码双方建立一致的类型/ID 约定；不应从字节内容推断分母。未开启 feature 时相应派生 trait 不存在。基础库可用并不等于 fory 及其依赖已在目标平台验证。

## 数学方法（math）

以下方法在 FixedI64<D> 和 FixedU64<D> 上均存在。全部按值接收 self，返回 Result<Self, MathError>，保持类别与分母；不是把结果先转为 f64，也不使用浮点输入的十进制 to_string() 路径。

### 全部方法与定义域

| 数学含义 | fast 档 | 普通档 | 严格档 | 定义域 |
| --- | --- | --- | --- | --- |
| e 的 x 次方 | fast_exp() | exp() | exp_strict() | 所有可表示输入 |
| 2 的 x 次方 | fast_exp2() | exp2() | exp2_strict() | 所有可表示输入 |
| 自然对数 | fast_ln() | ln() | ln_strict() | x > 0 |
| 以 2 为底对数 | fast_log2() | log2() | log2_strict() | x > 0 |
| 以 10 为底对数 | fast_log10() | log10() | log10_strict() | x > 0 |
| 指定底数对数 | fast_log(base: Self) | log(base: Self) | log_strict(base: Self) | x > 0，base > 0 且 base != 1 |
| 整数次幂 | fast_powi(n: i32) | powi(n: i32) | powi_strict(n: i32) | 除 0 的负次幂外均合法 |

log 的 base 必须与 self 同类别、同分母，不使用 FixedOperand。底数可以在 (0, 1)，其对数分母为负；实现按区间符号正确处理。数学域非法返回 Domain，而不是 ArithmeticError::DivisionByZero。

### 三档精度与保证

| 项目 | fast | 普通 | 严格 |
| --- | --- | --- | --- |
| 入口命名 | fast_ 前缀 | 无前后缀 | _strict 后缀 |
| 本库后端请求精度 | 固定 128 bit | 128 bit 起，每轮翻倍 | 192 bit 起，每轮翻倍 |
| 求值方式 | 单值近似，后端请求 ToEven | 向下/向上定向舍入的区间 | 向下/向上定向舍入的区间 |
| raw 量化 | 对近似值乘 D 后，半值远离零 | 量化区间上下界 | 量化区间上下界 |
| 成功保证 | 无统一数学误差界 | 与正确半值远离零舍入的 raw 相差至多 1 | 唯一、正确的半值远离零舍入 raw |
| 自适应升精度/回退 | 无 | 有升精度 | 有升精度 |
| PrecisionExhausted | 不会返回 | 可能 | 可能 |

普通档在上下界量化相同，或两端均可表示且相差至多 1 raw 时接受，后一种情况返回较小端的量化 raw。这个保证是相对“正确舍入的 raw”，不要改写为相对精确实数至多 1/D。

精度预算上限为 12_288 bit；普通档实际尝试 128、256、512、1024、2048、4096、8192，下一轮 16384 超预算即停止；严格档为 192、384、768、1536、3072、6144、12288。因此“上限 12288”不表示普通档也一定请求这一精度。

严格档要求区间两端落到同一个舍入 raw；若到预算上限仍跨越半值阈值或范围边界，返回 PrecisionExhausted。合法输入、实际答案可表示，并不保证严格档总能证明成功。

fast 档不计算证明区间，不因难以确认舍入而自动切换普通或严格档。128 bit 指本库发出的算术/函数请求精度，不限制后端内部保护位或内部工作量。不要将 fast 名称理解为已测得的固定加速倍数或固定运行时间。接近半值、raw 上下界或 log 底数接近 1 时，其结果或错误可能与其他档位不同。

### 定义域之外的范围与失败

数学域合法之后还要检查结果的 raw 范围。例如 FixedU64<100> 的 0.5 可以作为 ln 输入，但 ln(0.5) 的负 raw 无法表示，返回 Overflow；若负结果正确舍入成 0，则可成功。数学方法没有饱和版本。

普通/严格档只有能证明舍入结果越界才报 Overflow；无法完成范围/舍入证明可能报 PrecisionExhausted。fast 则对量化后的近似 raw 检查范围，或使用独立数学界提前证明溢出；它的 Overflow 不提供正确舍入证明。

exp/exp2 的所有档位在 x > 100 时凭独立界返回 Overflow，在 x < -100 时返回 raw=0；这不是对 powi 指数的限制。其他范围仍可能按实际量化结果溢出或舍入为零。

PrecisionExhausted 也可能来自当前精度下 log 的 ln(base) 区间仍含零，或负幂取倒数前正数区间的下界仍为零；普通/严格档会继续提高精度直到预算耗尽。

后端错误、NaN、无独立数学界的无穷、非法区间等返回 ComputationFailed。尤其负幂需要先计算正幂再取倒数：中间正幂无效/无穷，不能静默倒数成“精确零”；即使最终数学值很小，也可能失败。fast 的近似 ln(base) 为零或取倒数前中间幂为零也归为 ComputationFailed。fast 不会返回 PrecisionExhausted。

### 负幂、i32::MIN 与零

- powi 系列支持所有 i32 指数，包括 i32::MIN；指数幅度使用 unsigned_abs，不会直接对 i32::MIN 求有符号相反数。
- n < 0 表示倒数；负底数按指数奇偶恢复符号，奇数结果为负，偶数为正。
- 0 的负次幂返回 Domain。
- 定义 0^0 = 1；所有零次幂的结果为 raw=D，不是 raw=1。
- 接受 i32::MIN 不代表对所有底数都成功；范围、预算或中间计算仍可能失败。指数幅度转 usize 的实现面向可容纳该幅度的 32/64 位环境，不能由此宣称任意字长平台均支持。

```rust
use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};
type N = FixedI64<1_000_000>;
let one = N::try_from_integer(1).unwrap();
assert_eq!(one.exp_strict().unwrap().raw(), 2_718_282);
assert_eq!(N::try_from_integer(2).unwrap().powi_strict(-3).unwrap().raw(), 125_000);
assert_eq!(N::from_raw(0).powi_strict(0).unwrap().raw(), 1_000_000);
assert_eq!(N::from_raw(0).powi(-1), Err(MathError::Domain));
assert_eq!(FixedI64::<1>::from_raw(-2).powi_strict(-1).unwrap().raw(), -1);
assert_eq!(FixedI64::<100>::from_raw(200).fast_powi(-1).unwrap().raw(), 50);
assert_eq!(FixedI64::<100>::from_raw(-100).fast_powi(i32::MIN).unwrap().raw(), 100);
assert_eq!(FixedU64::<100>::from_raw(50).ln_strict(), Err(MathError::Overflow));
let eight = N::try_from_integer(8).unwrap();
let two = N::try_from_integer(2).unwrap();
assert_eq!(eight.log_strict(two).unwrap().raw(), 3_000_000);
```

## 验证命令

在仓库根目录、具备所需目标工具链与依赖环境时运行：

```sh
cargo test
cargo test --features math
cargo test --features serde
cargo test --features fory
cargo test --all-features
cargo test --doc --features math
cargo doc --no-deps --features math
```

部分集成测试直接启动 PATH 中的 rustc，编译临时库/二进制并检查编译失败诊断；需可用的编译器、链接器及临时目录权限。可选依赖的构建失败应结合当前 target、工具链和依赖要求判断，不能据基础测试推断所有组合平台都已支持。

README 和本文件是独立 Markdown，Cargo 不会自动把其中的 Rust 代码块收集为 doctest；cargo test --doc 只验证 Rust 文档注释收集到的样例，其成功不等于这些 Markdown 代码块被执行。以上示例已按当前 API 签名核对；含 math、serde_json、fory 的示例分别需要相应 feature 或使用方依赖。
