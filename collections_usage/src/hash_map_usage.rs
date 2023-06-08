/*
Hash Map

哈希 map 和所有权
对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。


*/
use std::collections::HashMap;

pub fn test_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，
    // 接着调用 unwrap_or 在 score 中没有该键所对应的项时将其设置为零。
    let team_name = String::from("Blue");
    let score1 = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 更新操作
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // 只在键没有对应值时插入键值对
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
}