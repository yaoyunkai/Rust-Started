/*
cfg 是 Rust 中用于条件编译的关键字，用于根据特定条件选择性地包含或排除代码块进行编译。

#[derive(Clone, Debug, PartialEq)]


为了分隔这两种参数，需要先列出传递给 cargo test 的参数，接着是分隔符 --，再之后是传递给测试二进制文件的参数。
运行 cargo test --help 会提示 cargo test 的有关参数，而运行 cargo test -- --help 可以提示在分隔符之后使用的有关参数。

cargo test -- --show-output

*/


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::add(2, 2);
        assert_eq!(result, 4);
        // assert_eq!(super::add(usize::MAX, 2), usize::MAX + 2);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = super::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = super::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = super::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = super::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }


    #[test]
    #[should_panic]
    fn greater_than_100() {
        super::Guess::new(200);
    }
}
