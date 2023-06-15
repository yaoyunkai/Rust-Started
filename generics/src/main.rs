use crate::lifetime::test_longest;
use crate::lifetime::test_longest2;

mod generics;
mod struct_generics;
mod traits;
mod lifetime;


fn main() {
    // println!("Hello, world!");

    // get_max();
    // get_max_v2();

    // test_largest_function();

    // test_generic_functions();

    // test_point();

    // test_use_trait();

    // test_dangling_references();

    test_longest();
    // test_longest2();

    // test_borrow();
}


fn test_borrow() {
    /*
    在Rust中，通过可变借用（mutable borrowing）获取的引用是不允许直接修改被借用的值的。
    为了修改被借用的值，需要使用解引用操作符*来取消引用，获取到实际的可变引用，然后进行修改操作。

    mut_borrow是一个可变借用的引用，它是一个指向s的可变引用。

    */
    let mut s = String::from("Hello");
    let borrow = &s; // 不可变借用
    println!("Borrowed: {}", borrow); // 可以通过借用读取值

    let mut_borrow = &mut s; // 可变借用
    *mut_borrow = String::from("World"); // 可以通过可变借用修改值
    println!("Modified: {}", s); // 输出 "Modified: World"
}
