/*
trait
相当于接口 interface

但是不能为外部类型实现外部 trait

这个限制是被称为 相干性（coherence）的程序属性的一部分，
或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。

如果想要对 NewsArticle 实例使用这个默认实现，
可以通过 impl Summary for NewsArticle {} 指定一个空的 impl 块。

*/


use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub fn test_use_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}


pub fn notify1(item: &impl Summary) {
    /*
    将trait作为参数

    */
    println!("Breaking news! {}", item.summarize());
}


pub fn notify2<T: Summary>(item: &T) {
    /*
    Trait Bound 语法

    */
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify4<T: Summary>(item1: &T, item2: &T) {}

pub fn notify5(item: &(impl Summary + Display)) {}

pub fn notify6<T: Summary + Display>(item: &T) {
    println!("Haha, {}", item.summarize());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    0
}


// TODO 为什么不需要 & 符号
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// fn returns_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    /*
    通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法。

    */
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
