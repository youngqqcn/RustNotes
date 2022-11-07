// Author: yqq
// Date: 2022-11-07 22:06:56
// Description: 闭包



fn main() {
    // println!("hello world");


    fn  func (i : i32) -> i32 { i * 2 }


    // “闭包类型”
    let func2 = |i : i32| -> i32 { i + 2};
    let func3  = |i     |    i + 1;

    println!("func(2) = {}", func(2));
    println!("func2(2) = {}", func2(2));
    println!("func3(2) = {}", func3(2));

    // 无参数闭包
    let o = || 1;
    println!("{}", o());
}