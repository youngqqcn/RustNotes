
// Author: yqq
// Date: 2022-11-14 22:04:43
// Description:  使用dyn返回trait

// Rust 编译器需要知道每个函数的返回类型需要多少空间。这意味着所有函数都必须返回一个具体类型。


struct Sheep {}

struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "bbbbbbbb"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mmmmmmmm"
    }
}

// 返回 Box<Animal>  没有 dyn也可以，2015版中只是会警告， 但是在Rust2021中会报错
fn random_animal(rnd: f64) -> Box<dyn Animal> {
    if rnd < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}


fn main() {
    let rnd = 0.234;

    let animal = random_animal(rnd);
    println!("{}", animal.noise());
}