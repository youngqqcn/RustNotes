// Author: yqq
// Date: 2022-11-20 16:58:51
// Description:

use std::mem;


#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point {
        x: 0.0,
        y: 0.0,
    }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}



fn main() {

    let p: Point = origin();

    let rect: Rectangle = Rectangle {
        p1: origin(),
        p2: Point {
            x: 3.0, y: 4.0
        }
    };

    let boxed_rect: Box<Rectangle> = Box::new(
        Rectangle {
            p1: origin(),
            p2: origin(),
        }
    );

    let boxed_point: Box<Point> = Box::new(origin());

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());


    println!("Point occupies {} bytes in the stack",
             mem::size_of_val(&p));
    println!("Rectangle occupies {} bytes in the stack",
             mem::size_of_val(&rect));

    // box 的宽度就是指针宽度
    println!("Boxed point occupies {} bytes in the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             mem::size_of_val(&boxed_rect));
    println!("Boxed box occupies {} bytes in the stack",
             mem::size_of_val(&box_in_a_box));


    // 将包含在 `boxed_point` 中的数据复制到 `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             mem::size_of_val(&unboxed_point));

}