Rust代码管理
--------------------------------------------

包 package: 相当于是一个项目
包中可以包含至多一个库 crate(library crate)。包中可以包含任意多个二进制 crate(binary crate)
    src/main.rs
    src/lib.rs

    src/bin/



crate:  Rust 在编译时最小的代码单位
    1. 二进制 包含 main.rs 文件
    2. 库
crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块

模块

路径


Cargo workspaces
