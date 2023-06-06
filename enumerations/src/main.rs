#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


fn main() {
    // println!("Hello, world!");

    // test_enum_with_struct();

    // test_enum_new_format();

    test_match2();
}


fn test_enum_value() {
    /*
    枚举的成员位于其标识符的命名空间中

    */
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

fn test_enum_with_struct() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home    : {:?}", home);
    println!("loopback: {:?}", loopback);
}

fn test_enum_new_format() {
    /*
    每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数。

    结构体和枚举还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法。
    */

    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    dbg!(home);
}


fn test_option() {
    /*
    在对 Option<T> 进行运算之前必须将其转换为 T

    */

    let some_number: Option<i32> = Option::Some(5);
    let some_char: Option<char> = Option::Some('e');
    let absent_number: Option<i32> = Option::None;
    // 类型相同
    // let sum2 = some_number + absent_number;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // 类型不同 不能相加
    // let sum = x + y;
}


// ---------------------------------------------------------

fn test_match() {
    /*
    每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。

    */
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    impl Coin {}

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let num = value_in_cents(Coin::Dime);
}


fn test_match2() {
    #[derive(Debug)] // 这样可以立刻看到州的名称
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));
}


fn test_match_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn test_match_pattern() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // ----------- mode 2 -----------------

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn reroll() {}

    // ----------- mode 3 -----------------

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

// -------------------------------------------------------------------

fn test_if_let() {
    let config_max = Option::Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // 换句话说，可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // -------------------------------------------------------

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
