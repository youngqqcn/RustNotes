// Author: yqq
// Date: 2022-11-04 23:01:04
// Description: 常量

// Rust中有两种常量 const、static
//  'static 生命周期的，可以是可变的变量  'static mut


// 不可修改的静态常量
static LAN: &'static str = "Rust";


const COUNT: i32 = 100;


static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;


fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    // THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}