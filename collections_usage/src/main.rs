use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;

use crate::hash_map_usage::test_hash_map;

mod vector_usage;
mod string_usage;
mod hash_map_usage;

fn main() {
    // println!("Hello, world!");

    // 使用 vector
    // create_vector();
    // update_vector();
    // read_for_vector();
    // test_iter_vector();
    // update_string();

    // test_string_index();
    // test_string_slice();
    // test_iter_string();

    test_hash_map();

    // test_open_file();
}


fn test_open_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 直接借用
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn test_open_file2() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn test_open_file3() {
    /*
    如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
    如果 Result 是成员 Err，unwrap 会为我们调用 panic!。

    使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。

    */
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}


fn read_username_from_file() -> Result<String, io::Error> {
    /*
    当编写一个其实先会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，
    还可以选择让调用者知道这个错误并决定该如何处理。这被称为 传播（propagating）错误
    */

    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_simple() -> Result<String, io::Error> {
    /*
    传播错误的简写: ?
    ? 运算符只能被用于返回值与 ? 作用的值相兼容的函数。

    From trait

    */
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}