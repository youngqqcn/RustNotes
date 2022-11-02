// Author: yqq
// Date: 2022-11-02 22:45:09
// Description: Display trait 实现自定义输出


// https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_display.html

// 作业实现


use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    image: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.image)
    }
}


fn main() {
    let complex = Complex{real: 3.3, image: 7.2};
    println!("{}", complex);
    println!("{:?}", complex);
}

