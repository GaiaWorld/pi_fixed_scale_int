# pi-fixed-scale-int · context（L0）

- 地形：Cargo.toml:1-7 为 Rust 2024、无依赖；src/main.rs:1-3 仅为默认可执行入口。无库接口与测试布局，先前 cargo test 通过但没有测试用例。
- 素材：已读 Cargo.toml、src/main.rs 和本轮对话；无其他未读素材。
- 来源：对话为行为目标与澄清；文件为项目现状。没有可达的组织既有方案，不推断库接口惯例。
- 待验证：无。L7 用 cargo build --lib 构建库后，分别以 rustc --edition=2024 编译使用零分母构造、读取、比较和目标分母转换的四个独立样例，均因 const assert 失败退出（exit=1）；仅声明未使用的类型及派生操作不在验证范围。
