/*
当 vector 被丢弃时，所有其内容也会被丢弃，这意味着这里它包含的整数将被清理。
借用检查器确保了任何 vector 中内容的引用仅在 vector 本身有效时才可用。

*/

pub fn create_vector() {
    // let v: Vec<i32> = Vec::new();

    // vec! 宏来创建vector
    let v = vec![1, 2, 3];
    println!("elements of v: {:#?}", v);
}

pub fn update_vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("elements of v: {:#?}", v);
}

pub fn read_for_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // 索引访问越界会导致程序panic
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

pub fn test_borrow_vector() {
    /*
    在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，
    可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。

    */
    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];
    //
    // v.push(6);

    // println!("The first element is: {first}");
}

pub fn test_iter_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}

pub fn test_use_enum_with_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}