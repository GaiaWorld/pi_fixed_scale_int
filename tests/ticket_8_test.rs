#[path = "support/ticket_8_test_inner.rs"]
mod ticket_8_test_inner;

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: mixed_fixed_mul
 *   Given 左值 raw150/D100，异类右值 raw5/D10
 *   When 普通混合乘法（含 _with 别名）
 *   Then 保持左值类型与D100，raw=75，两源值不变
 *   Examples: 左U/右I；左I/右U
 */
#[test]
fn mixed_fixed_mul() {
    ticket_8_test_inner::mixed_fixed_mul();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: primitive_mul
 *   Given 两类左值 raw150/D100，原生整数右值100
 *   When 普通混合乘法
 *   Then 原类型D100的raw=15000，源值不变
 *   Examples: 右类型 u8/u16/u32/u64/i8/i16/i32/i64；左U/I
 */
#[test]
fn primitive_mul() {
    ticket_8_test_inner::primitive_mul();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: near_zero_mul_checked
 *   Given D10左值raw ±1；D100右值在半值两侧或恰为半值
 *   When 向零截断混合乘法
 *   Then 两类结果raw均为0，负真值亦向零截断，两源值不变
 *   Examples: 右raw绝对值49/50/51；I±1配U正右值；I±1配I负右值；U1配I正/负右值；期望raw均0
 */
#[test]
fn near_zero_mul_checked() {
    ticket_8_test_inner::near_zero_mul_checked();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: near_zero_mul_rounded
 *   Given D10左值raw ±1；D100右值在半值两侧或恰为半值
 *   When 半值远离零混合乘法
 *   Then 有符号结果按符号返回0或±1；无符号负结果舍入为0成功、舍入为-1报Overflow；两源值不变
 *   Examples: 右raw绝对值49/50/51；I±1配U正右值；I±1配I负右值；U1配I正/负右值；期望绝对raw依次0/1/1
 */
#[test]
fn near_zero_mul_rounded() {
    ticket_8_test_inner::near_zero_mul_rounded();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: negative_unsigned_mul_checked
 *   Given 无符号raw150/D100，右值raw-5/D10或-2i8
 *   When checked混合乘法
 *   Then 返回Overflow，源值不变
 *   Examples: 右值 I<-5,D10>；-2i8
 */
#[test]
fn negative_unsigned_mul_checked() {
    ticket_8_test_inner::negative_unsigned_mul_checked();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: negative_unsigned_mul_rounded
 *   Given 无符号raw150/D100，右值raw-5/D10或-2i8
 *   When rounded混合乘法
 *   Then 返回Overflow，源值不变
 *   Examples: 右值 I<-5,D10>；-2i8
 */
#[test]
fn negative_unsigned_mul_rounded() {
    ticket_8_test_inner::negative_unsigned_mul_rounded();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: negative_unsigned_mul_saturating
 *   Given 无符号raw150/D100，右值raw-5/D10或-2i8
 *   When saturating混合乘法
 *   Then 夹到raw0，源值不变
 *   Examples: 右值 I<-5,D10>；-2i8
 */
#[test]
fn negative_unsigned_mul_saturating() {
    ticket_8_test_inner::negative_unsigned_mul_saturating();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: range_mul_checked
 *   Given D10左值为u64::MAX、i64::MAX或i64::MIN，右值为原生整数2u8；另有D10的i64::MIN与原生整数-1i8
 *   When checked混合乘法
 *   Then 返回Overflow，两源值不变
 *   Examples: U MAX配2u8；I MAX配2u8；I MIN配2u8；I MIN配-1i8
 */
#[test]
fn range_mul_checked() {
    ticket_8_test_inner::range_mul_checked();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: wide_mul_checked
 *   Given D100左值为u64::MAX或i64::MIN，异类右值raw=D=u32::MAX（数值1）
 *   When checked混合乘法
 *   Then 保持原类型D100和极值raw，两源值不变，中间积不误报越界
 *   Examples: 左U MAX/右I；左I MIN/右U
 */
#[test]
fn wide_mul_checked() {
    ticket_8_test_inner::wide_mul_checked();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: range_mul_rounded
 *   Given D10左值为u64::MAX、i64::MAX或i64::MIN，右值为原生整数2u8；另有D10的i64::MIN与原生整数-1i8
 *   When rounded混合乘法
 *   Then 返回Overflow，两源值不变
 *   Examples: U MAX配2u8；I MAX配2u8；I MIN配2u8；I MIN配-1i8
 */
#[test]
fn range_mul_rounded() {
    ticket_8_test_inner::range_mul_rounded();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: wide_mul_rounded
 *   Given D100左值为u64::MAX或i64::MIN，异类右值raw=D=u32::MAX（数值1）
 *   When rounded混合乘法
 *   Then 保持原类型D100和极值raw，两源值不变，中间积不误报越界
 *   Examples: 左U MAX/右I；左I MIN/右U
 */
#[test]
fn wide_mul_rounded() {
    ticket_8_test_inner::wide_mul_rounded();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: range_mul_saturating
 *   Given D10左值为u64::MAX、i64::MAX或i64::MIN，右值为原生整数2u8；另有D10的i64::MIN与原生整数-1i8
 *   When saturating混合乘法
 *   Then 夹到对应边界，MIN与-1结果夹到i64::MAX，两源值不变
 *   Examples: U MAX配2u8；I MAX配2u8；I MIN配2u8；I MIN配-1i8
 */
#[test]
fn range_mul_saturating() {
    ticket_8_test_inner::range_mul_saturating();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: wide_mul_saturating
 *   Given D100左值为u64::MAX或i64::MIN，异类右值raw=D=u32::MAX（数值1）
 *   When saturating混合乘法
 *   Then 保持原类型D100和极值raw，两源值不变，中间积不误报越界
 *   Examples: 左U MAX/右I；左I MIN/右U
 */
#[test]
fn wide_mul_saturating() {
    ticket_8_test_inner::wide_mul_saturating();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: negative_primitive_mul
 *   Given 有符号raw150/D100与-2i8
 *   When 普通混合乘法
 *   Then 保持I<D100>且raw=-300，源值不变
 *   Examples: 左I150/D100；右-2i8
 */
#[test]
fn negative_primitive_mul() {
    ticket_8_test_inner::negative_primitive_mul();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: rounding_crosses_bound_mul
 *   Given 左值D10，精确结果raw为u64::MAX+1/2、i64::MAX+1/2或i64::MIN-1/2
 *   When 以截断及半值远离零模式进行同一混合乘法
 *   Then 截断分别得u64::MAX/i64::MAX/i64::MIN，舍入均报Overflow，两源值不变
 *   Examples: U1190112520884487201 ×I31/D2；I6148914691236517205 ×U3/D2；I-67280421310721 ×U274177/D2
 */
#[test]
fn rounding_crosses_bound_mul() {
    ticket_8_test_inner::rounding_crosses_bound_mul();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: mixed_fixed_div
 *   Given 左值 raw150/D100，异类右值 raw5/D10
 *   When 普通混合除法（含 _with 别名）
 *   Then 保持左值类型与D100，raw=300，两源值不变
 *   Examples: 左U/右I；左I/右U
 */
#[test]
fn mixed_fixed_div() {
    ticket_8_test_inner::mixed_fixed_div();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: primitive_div
 *   Given 两类左值 raw150/D100，原生整数右值2
 *   When 普通混合除法
 *   Then 原类型D100的raw=75，源值不变
 *   Examples: 右类型 u8/u16/u32/u64/i8/i16/i32/i64；左U/I
 */
#[test]
fn primitive_div() {
    ticket_8_test_inner::primitive_div();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: near_zero_div_checked
 *   Given D10左值raw ±1；D100右值在半值两侧或恰为半值
 *   When 向零截断混合除法
 *   Then 两类结果raw均为0，负真值亦向零截断，两源值不变
 *   Examples: 右raw绝对值201/200/199；I±1配U正右值；I±1配I负右值；U1配I正/负右值；期望raw均0
 */
#[test]
fn near_zero_div_checked() {
    ticket_8_test_inner::near_zero_div_checked();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: near_zero_div_rounded
 *   Given D10左值raw ±1；D100右值在半值两侧或恰为半值
 *   When 半值远离零混合除法
 *   Then 有符号结果按符号返回0或±1；无符号负结果舍入为0成功、舍入为-1报Overflow；两源值不变
 *   Examples: 右raw绝对值201/200/199；I±1配U正右值；I±1配I负右值；U1配I正/负右值；期望绝对raw依次0/1/1
 */
#[test]
fn near_zero_div_rounded() {
    ticket_8_test_inner::near_zero_div_rounded();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: negative_unsigned_div_checked
 *   Given 无符号raw150/D100，右值raw-5/D10或-2i8
 *   When checked混合除法
 *   Then 返回Overflow，源值不变
 *   Examples: 右值 I<-5,D10>；-2i8
 */
#[test]
fn negative_unsigned_div_checked() {
    ticket_8_test_inner::negative_unsigned_div_checked();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: negative_unsigned_div_rounded
 *   Given 无符号raw150/D100，右值raw-5/D10或-2i8
 *   When rounded混合除法
 *   Then 返回Overflow，源值不变
 *   Examples: 右值 I<-5,D10>；-2i8
 */
#[test]
fn negative_unsigned_div_rounded() {
    ticket_8_test_inner::negative_unsigned_div_rounded();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: negative_unsigned_div_saturating
 *   Given 无符号raw150/D100，右值raw-5/D10或-2i8
 *   When saturating混合除法
 *   Then 夹到raw0，源值不变
 *   Examples: 右值 I<-5,D10>；-2i8
 */
#[test]
fn negative_unsigned_div_saturating() {
    ticket_8_test_inner::negative_unsigned_div_saturating();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: range_div_checked
 *   Given D10左值为u64::MAX、i64::MAX或i64::MIN，右值为异类raw1/D2（数值1/2）；另有D10的i64::MIN与原生整数-1i8
 *   When checked混合除法
 *   Then 返回Overflow，两源值不变
 *   Examples: U MAX配I raw1/D2；I MAX配U raw1/D2；I MIN配U raw1/D2；I MIN配-1i8
 */
#[test]
fn range_div_checked() {
    ticket_8_test_inner::range_div_checked();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: wide_div_checked
 *   Given D100左值为u64::MAX或i64::MIN，异类右值raw=D=u32::MAX（数值1）
 *   When checked混合除法
 *   Then 保持原类型D100和极值raw，两源值不变，中间积不误报越界
 *   Examples: 左U MAX/右I；左I MIN/右U
 */
#[test]
fn wide_div_checked() {
    ticket_8_test_inner::wide_div_checked();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: range_div_rounded
 *   Given D10左值为u64::MAX、i64::MAX或i64::MIN，右值为异类raw1/D2（数值1/2）；另有D10的i64::MIN与原生整数-1i8
 *   When rounded混合除法
 *   Then 返回Overflow，两源值不变
 *   Examples: U MAX配I raw1/D2；I MAX配U raw1/D2；I MIN配U raw1/D2；I MIN配-1i8
 */
#[test]
fn range_div_rounded() {
    ticket_8_test_inner::range_div_rounded();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: wide_div_rounded
 *   Given D100左值为u64::MAX或i64::MIN，异类右值raw=D=u32::MAX（数值1）
 *   When rounded混合除法
 *   Then 保持原类型D100和极值raw，两源值不变，中间积不误报越界
 *   Examples: 左U MAX/右I；左I MIN/右U
 */
#[test]
fn wide_div_rounded() {
    ticket_8_test_inner::wide_div_rounded();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: range_div_saturating
 *   Given D10左值为u64::MAX、i64::MAX或i64::MIN，右值为异类raw1/D2（数值1/2）；另有D10的i64::MIN与原生整数-1i8
 *   When saturating混合除法
 *   Then 夹到对应边界，MIN与-1结果夹到i64::MAX，两源值不变
 *   Examples: U MAX配I raw1/D2；I MAX配U raw1/D2；I MIN配U raw1/D2；I MIN配-1i8
 */
#[test]
fn range_div_saturating() {
    ticket_8_test_inner::range_div_saturating();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: wide_div_saturating
 *   Given D100左值为u64::MAX或i64::MIN，异类右值raw=D=u32::MAX（数值1）
 *   When saturating混合除法
 *   Then 保持原类型D100和极值raw，两源值不变，中间积不误报越界
 *   Examples: 左U MAX/右I；左I MIN/右U
 */
#[test]
fn wide_div_saturating() {
    ticket_8_test_inner::wide_div_saturating();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: negative_primitive_div
 *   Given 有符号raw150/D100与-2i8
 *   When 普通混合除法
 *   Then 保持I<D100>且raw=-75，源值不变
 *   Examples: 左I150/D100；右-2i8
 */
#[test]
fn negative_primitive_div() {
    ticket_8_test_inner::negative_primitive_div();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: rounding_crosses_bound_div
 *   Given 左值D10，精确结果raw为u64::MAX+1/2、i64::MAX+1/2或i64::MIN-1/2
 *   When 以截断及半值远离零模式进行同一混合除法
 *   Then 截断分别得u64::MAX/i64::MAX/i64::MIN，舍入均报Overflow，两源值不变
 *   Examples: U1190112520884487201 ÷I2/D31；I6148914691236517205 ÷U2/D3；I-67280421310721 ÷U2/D274177
 */
#[test]
fn rounding_crosses_bound_div() {
    ticket_8_test_inner::rounding_crosses_bound_div();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: zero_divisor_checked
 *   Given 两类raw150/D100，异类D10零或原生整数零
 *   When checked混合除法及_with别名
 *   Then 返回DivisionByZero，两源值不变
 *   Examples: U/I0；I/U0；U/0i8；I/0u64
 */
#[test]
fn zero_divisor_checked() {
    ticket_8_test_inner::zero_divisor_checked();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: zero_divisor_rounded
 *   Given 两类raw150/D100，异类D10零或原生整数零
 *   When rounded混合除法及_with别名
 *   Then 返回DivisionByZero，两源值不变
 *   Examples: U/I0；I/U0；U/0i8；I/0u64
 */
#[test]
fn zero_divisor_rounded() {
    ticket_8_test_inner::zero_divisor_rounded();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/fixed_inner.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: zero_divisor_saturating
 *   Given 两类raw150/D100，异类D10零或原生整数零
 *   When saturating混合除法及_with别名
 *   Then 返回DivisionByZero，两源值不变
 *   Examples: U/I0；I/U0；U/0i8；I/0u64
 */
#[test]
fn zero_divisor_saturating() {
    ticket_8_test_inner::zero_divisor_saturating();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: serde_raw_round_trip
 *   Given serde启用，最大u32分母的两类raw边界
 *   When 经serde_json编码再解码为原类型
 *   Then raw和源值均保持，不限制字节格式或要求协议互通
 *   Examples: U:0/1/18446744073709551615；I:-9223372036854775808/-1/0/9223372036854775807
 */
#[cfg(feature = "serde")]
#[test]
fn serde_raw_round_trip() {
    ticket_8_test_inner::serde_raw_round_trip();
}

/** 契约：design/pi-fixed-scale-int/07-design/call-chain.md 切片8；design/pi-fixed-scale-int/01-requirements/needs.md「加减比较与混合乘除」「舍入方式」「失败与饱和」「十进制输入、输出与序列化」（未分配 API ID）
 * @api:src/lib.rs
 * @impl:src/lib.rs
 * @kind: contract
 * @edgecase
 * Scenario Outline: fory_raw_round_trip
 *   Given fory启用，最大u32分母的两类raw边界
 *   When 经fory编码再解码为原类型
 *   Then raw和源值均保持，不限制字节格式或要求协议互通
 *   Examples: U:0/1/18446744073709551615；I:-9223372036854775808/-1/0/9223372036854775807
 */
#[cfg(feature = "fory")]
#[test]
fn fory_raw_round_trip() {
    ticket_8_test_inner::fory_raw_round_trip();
}

