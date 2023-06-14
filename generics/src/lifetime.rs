/*
生命周期

移动 move
克隆 clone
    如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。
    同理还有 Drop trait

拷贝 Copy 只在栈上的数据


变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。


引用与借用

& 使用引用
* 解引用

我们将创建一个引用的行为称为 借用（borrowing）

在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），
所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。


------------------------------------------------------------------------------------


Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域。

借用检查器（borrow checker）


生命周期注解并不改变任何引用的生命周期的长短。
相反它们描述了多个引用生命周期相互的关系，而不影响其生命周期。
与当函数签名中指定了泛型类型参数后就可以接受任何类型一样，当指定了泛型生命周期后函数也能接受任何生命周期的引用。


*/


pub fn test_dangling_references() {
    let r;

    // println!("the value of r: {}", r);
    {
        let x = 5;
        r = &x;
        println!("r: {}", r);
    }
    // println!("r: {}", r);
}


/*

fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+


fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}


*/

pub fn test_longest2() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest_new(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}


pub fn test_longest() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_new(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// fn longest(x: &str, y: &str) -> &str {
//     /*
//     提示文本揭示了返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y。
//     事实上我们也不知道，因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！
//
//     */
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest_new<'a, >(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
