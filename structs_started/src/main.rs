// 这是因为结构体定义本身被视为一个完整的语法结构，并不需要添加额外的分号。
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct User1 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }


// 这是因为它们是定义类型的一部分，而不是一个单独的语法结构。
// 在Rust中，类型定义通常以分号结尾。
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // println!("user email: {}", user1.email);

    // test_struct_update_syntax();
}


fn build_user(email: String, username: String) -> User {
    /*
    字段初始化简写语法（field init shorthand）
    */

    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


fn test_struct_update_syntax() {
    /*
    结构体更新语法（struct update syntax）

    */

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        // 相当于只借用了两个栈变量

        email: String::from("another@example.com"),
        username: String::from("xxxxxxxxxxxx"),
        ..user1
    };

    let user3 = User {
        // 这里借用了所有的变量，包括堆变量，那么 user1的 email和username就被move到user3上面了
        ..user1
    };

    println!("{}", user2.username);
    println!("{}", user3.username);
}

fn test_tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// -------------------------------------------------------------

