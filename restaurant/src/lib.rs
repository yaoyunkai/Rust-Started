/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

-------------------------------------------------------------
在 Rust 中，默认所有项（函数、方法、结构体、枚举、模块和常量）对父模块都是私有的。

引用模块项目的路径
绝对路径（absolute path）是以 crate 根（root）开头的全路径；对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于对于当前 crate 的代码，则以字面值 crate 开头。
相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。

使用 use 将两个同名类型引入同一作用域这个问题还有另一个解决办法：在这个类型的路径后面，我们使用 as 指定一个新的本地名称或者别名。
use std::fmt::Result;
use std::io::Result as IoResult;


使用 pub use 重导出名称: re-exporting

use std::{cmp::Ordering, io};

use std::collections::*; 引入所有的公共定义


*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


// TODO 这个函数和上面模块的关系是什么
pub fn eat_at_restaurant() {
    /*
    因为 eat_at_restaurant 函数与 front_of_house 定义于同一模块中
    我们可以从 eat_at_restaurant 中引用 front_of_house。

    */

    // // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    //
    // // 相对路径
    // front_of_house::hosting::add_to_waitlist();


    // 注意 use 只能创建 use 所在的特定作用域内的短路径。
    use crate::front_of_house::hosting;

    hosting::add_to_waitlist();
}

fn deliver_order() {
    // fix_incorrect_order();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        /*
        如果我们在一个结构体定义的前面使用了 pub ，
        这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。
        */
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}


pub fn eat_at_restaurant2() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
