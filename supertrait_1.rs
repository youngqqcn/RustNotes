


use std::fmt;


// 实现  supertrait
trait OutlinePrint: fmt::Display { // 相当于为 trait 增加 trait bound
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}


struct Point {
    x: i32,
    y: i32,
}


impl OutlinePrint for Point {
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn main() {
    let p = Point {x: 55, y: 99};

    // println!("{}", p);
    p.outline_print();
}