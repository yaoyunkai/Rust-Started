/*
结构体泛型

Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

*/


struct Point<T> {
    x: T,
    y: T,
}

impl<E> Point<E> {
    fn x(&self) -> &E {
        &self.x
    }
}


impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Pair<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Pair<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Pair<X2, Y2>) -> Pair<X1, Y2> {
        Pair {
            x: self.x,
            y: other.y,
        }
    }
}


pub fn test_point() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // mixup
    let p1 = Pair { x: 5, y: 10.4 };
    let p2 = Pair { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
