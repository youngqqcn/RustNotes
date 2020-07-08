use std::cmp::{PartialOrd, PartialEq} ;
// use std::mem::


// 在函数定义中使用泛型
fn largest<T: PartialOrd>(v: &[T]) -> &T {
    let mut max = &v[0];
    for item in v.iter() {
        if item > &max {
            max  = item;
        }
    }
    max
}


//结构体中使用泛型
struct Point<X, Y> {
    x: X, 
    y: Y,
}

impl<X, Y> Point<X, Y> {
    // 方法中使用泛型
    fn mixup<XX, YY>(self, other: Point<XX, YY>) -> Point<X, YY> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}




fn main() {

    let v = vec![1, -1, 3, 9, 4];
    if let x =  largest(&v) {
        println!("x = {}", x);
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
