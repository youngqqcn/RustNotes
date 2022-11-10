// Author: yqq
// Date: 2022-11-10 22:17:24
// Description: 约束

use std::fmt;

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64
}

#[derive(Debug)]
struct Triangle {
    length: f64,
    height: f64,
}

// 泛型 `T` 必须实现 `Debug` 。只要满足这点，无论什么类型
// 都可以让下面函数正常工作。
fn print_debug<T: fmt::Debug>(t: &T) {
    println!("{:?}", t);
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn get_area<T: HasArea>(t: &T) -> f64 {
    t.area()
}


fn main() {

    let rec = Rectangle { length: 3.0, height: 4.0 };
    let tri = Triangle { length: 2.0, height: 2.0 };

    print_debug(&rec);
    print_debug(&tri);


    println!("{}", get_area(&rec));

    // println!("{}", get_area(&tri));
}