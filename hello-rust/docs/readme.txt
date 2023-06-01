Rustacean

--------------------------------------------------

cargo new <dir name>

cargo build

cargo run

cargo check  在不生成二进制文件的情况下构建项目来检查错误。

cargo doc --open 来构建所有本地依赖提供的文档

--------------------------------------------------

预导入 preclude

use  ->> import


let apples = 5; // 不可变
let mut bananas = 5; // 可变

::new 那一行的 :: 语法表明 new 是 String 类型的一个 关联函数（associated function）。
关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个特定实例。一些语言中把它称为 静态方法（static method）。

& 表示这个参数是一个 引用（reference）

Result类型


// fn main() {
//     // 调用了一个 Rust 宏（macro）: 看到 ！
//     println!("Hello, world!");
// }



--------------------------------------------------

trait

范围表达式 start..=end

Ordering enum : 也是一个枚举，不过它的成员是 Less、Greater 和 Equal

match 表达式

shadowing
