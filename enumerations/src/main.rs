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

    test_enum_new_format();
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
    let sum2 = some_number + absent_number;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // 类型不同 不能相加
    let sum = x + y;
}
