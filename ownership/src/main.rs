fn main() {
    // test_string();

    // test_variables_move();

    // test_variables_copy();

    // test_ownership_and_function();

    // test_return_value_and_ownership();

    // test_reference();

    // test_multi_reference();

    // test_create_twice_reference();

    // test_dangling_references();

    // test_get_first_word();
}

fn test_string() {
    /*
    - 必须在运行时向内存分配器（memory allocator）请求内存。
    - 需要一个当我们处理完 `String` 时将内存返回给分配器的方法。

    内存在拥有它的变量离开作用域后就被自动释放。
    当变量离开作用域时，rust会自动调用 drop

    */

    let s = String::from("hello");
    let num = s.len();
    println!("the s length is {}", num);

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
}

fn test_variables_move_int() {
    /*
    变量与数据交互的方式（一）：移动

    因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中。
    */

    let x = 5;
    let y = x;

    println!("the value x: {}, y: {}", x, y);
}

fn test_variables_move_string() {
    /*
    为了确保内存安全，在 let s2 = s1; 之后，
    Rust 认为 s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西。

    */

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);
}

fn test_variables_copy() {
    /*
    当出现 clone 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源。

    */

    let s1 = String::from("hello");
    let s2 = s1.clone();
    // let s2 = "hello world";

    println!("s1 = {}, s2 = {}", s1, s2);
}

// --------------------------------------------------------------------------------

fn test_ownership_and_function() {
    let s = String::from("hello"); // s 进入作用域
    println!("the variable before s: {}", s);

    takes_ownership(s); // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    // println!("the variable after  s: {}", s);

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    // 所以在后面可继续使用 x
    println!("the variable x: {}", x);
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
    // 这里，some_string 移出作用域并调用 `drop` 方法。
    // 占用的内存被释放
}

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
    // 这里，some_integer 移出作用域。没有特殊之处
}

// --------------------------------------------------------------------------------

fn test_return_value_and_ownership() {
    /*
    变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
    当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
    */

    let s1 = gives_ownership(); // gives_ownership 将返回值
    println!("S1 value: {}", s1);

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到

    // println!("S2 value: {}", s2);
    println!("S3 value: {}", s3);
}

fn gives_ownership() -> String {
    // gives_ownership 会将
    let some_string = String::from("yours"); // some_string 进入作用域。
    some_string // 返回 some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

// --------------------------------------------------------------------------------

fn test_reference() {
    let s1 = String::from("hello");
    // &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // TODO why ????
    println!("the String content: {}", &s);
    println!("the String content: {}", s);
    println!("the String content: {}", *s);
    s.len()
}

// --------------------------------------------------------------------------------

#[allow(dead_code)]
fn test_multi_reference() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("the string value is: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// --------------------------------------------------------------------------------

fn test_create_twice_reference() {
    /*
    如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。

    如果没有可变引用，可以一直创建不可变引用

    */

    let mut s = String::from("Rust!!!");

    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题
    // println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

// --------------------------------------------------------------------------------

fn test_dangling_references() {
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("the string is: {}", reference_to_nothing);
}

// fn dangle() -> &String {
//     /*
//     因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放。
//     不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String，
//     */
//     let s = String::from("hello");
//
//     &s
// }

fn no_dangle() -> String {
    /*
    这样就没有任何错误了。所有权被移动出去，所以没有值被释放。
    */

    let s = String::from("hello");

    s
}

// --------------------------------------------------------------------------------

fn test_get_first_word() {
    /*
    不得不时刻担心 word 的索引与 s 中的数据不再同步

    */

    let mut s = String::from("hello world");
    let num = first_word(&s);
    s.clear();
    println!("the length is: {}", num);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // println!("bytes is: {}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// --------------------------------------------------------------------------------

fn test_string_slice() {
    /*
    由中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice
    slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 ending_index 减去 starting_index 的值。

    字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，
    如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。

    */
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];
}

fn first_word_new(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
