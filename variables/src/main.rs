/*
变量的可变性

常量: 声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型。

shadowing

当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。

------------------------------------------------------------

data type
    标量 scalar
    复合 compound

类型注解

整型: i8 i16 i32 i64 i128 isize u8 u15 u32 u64 u128 usize
      isize 和 usize 类型依赖运行程序的计算机架构

浮点型: f32 f64

bool:

字符类型:
    char: 大小为四个字节 代表了一个 unicode 标量


------------------------------------------------------------

为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值
不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组。

数组 array  数组长度是固定的。

------------------------------------------------------------
在函数签名中，必须 声明每个参数的类型。
函数体由一系列的语句和一个可选的结尾表达式构成。
语句（Statements）是执行一些操作但不返回值的指令。 表达式（Expressions）计算并产生一个值。

!!!! 表达式的结尾没有分号。

*/


use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


fn main_old() {
    // println!("Hello, world!");

    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    println!("The value of x is: {x}");
}

fn main_old2() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}


fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // ------------------------------------------------------
    let c = 'z';
    let d: char = '你';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    let tuple1 = ();

    // ------------------------------------------------------

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let arr2 = [3.455; 5];

    println!("the first element of arr is: {}", arr[0]);
    println!("the first element of arr is: {}", arr[2]);

    // test_array_index();

    // ------------------------------------------------------

    another_function(45);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five2();

    println!("The value of x is: {x}");
}


fn test_array_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}


fn five() -> i32 {
    5
}

fn five2() -> i32 {
    return 5;
}


fn plus_one(x: i32) -> i32 {
    x + 1
}