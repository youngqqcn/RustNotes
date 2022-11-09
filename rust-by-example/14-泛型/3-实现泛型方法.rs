// Author: yqq
// Date: 2022-11-09 23:02:48
// Description: 泛型方法

struct Val {
    val: f64
}

struct GenVal<T> {
    gen_val: T
}

impl Val {
    fn value(&self) -> &f64 { &self.val}
}

// 泛型方法实现
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}

fn main() {
    let x = Val { val: 33333.3333 };
    let y = GenVal { gen_val: 333}; // 同效： GenVal::<i32> { gen_val: 333};

    println!("{}, {}", x.value(), y.value());
}
