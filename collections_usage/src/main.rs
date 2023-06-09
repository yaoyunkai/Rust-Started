use std::fs::File;
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

    test_open_file();
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
    */
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
