// Author: yqq
// Date: 2022-11-07 21:52:49
// Description:  方法


// 方法（method）是依附于对象的函数

struct Point {
    x: f64,
    y: f64,
}

impl Point {

    fn origin() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {

    fn area(&self) -> f64 {

        let Point{x: x1, y: y1} = self.p1;
        let Point{x: x2, y: y2} = self.p2;

        (x1 - x2).abs() * (y1 - y2).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point{x: x1, y: y1} = self.p1;
        let Point{x: x2, y: y2} = self.p2;

        ((x1 - x2).abs() + (y1 - y2).abs()) * 2f64
    }
}

#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法会 “消耗” 调用者的资源
    // `self` 为 `self: Self` 的语法糖
    fn destroy(self) {
        // 解构 `self`
        let Pair(first, second) = self; // self 被 move 了

        println!("Destroying Pair({}, {})", first, second);

        // `first` 和 `second` 离开作用域后释放
    }
}

fn main() {
    let rectangle = Rectangle {
        // 静态方法使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 实例方法通过点运算符来调用
    // 注意第一个参数 `&self` 是隐式传递的，亦即：
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };


    // 报错！ `rectangle` 是不可变的，但这方法需要一个可变对象
    //rectangle.translate(1.0, 0.0);
    // 试一试 ^ 去掉此行的注释

    // 正常运行！可变对象可以调用可变方法
    // square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // println!("{:?}", pair); // error: value borrowed here after move
}