/*
String 字符串是 UTF-8 编码

当 Rustacean 们谈到 Rust 的 “字符串”时，它们通常指的是 String 或字符串 slice &str 类型，而不特指其中某一个。


*/


pub fn test_string() {

    // String initialize
    let data = "initial contents";

    let mut s = String::new();

    let s = data.to_string();
    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();

    let s1 = String::from("initial contents");
}

pub fn update_string() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s3 = String::from("lo");
    s3.push('l');

    {
        // 因为 &String 可以被 强转（coerced）成 &str
        // Deref 强制转换（deref coercion）
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
        println!("S3 is {}", s3);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;

        // format 宏
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
    }
}

pub fn test_string_index() {
    let s1 = String::from("hello");
    // let h = s1[0];

    let hello = String::from("Здравствуйте");
    println!("hello len is {}", hello.len());
}

pub fn test_string_slice() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("the s is {}", s);

    // let s1 = &hello[0..1];
}

pub fn test_iter_string() {
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}


pub fn hello() {
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
