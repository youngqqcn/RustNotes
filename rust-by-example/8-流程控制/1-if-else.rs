// Author: yqq
// Date: 2022-11-06 11:31:19
// Description: Rust   if/else



fn main() {


    let x = 9;


    if 1 == (x & 1) {
        println!("odd number");
    } else {
        println!("even number");
    }

    let y =  if x < 9 {
        x * 100
    } else {
        // 表达式返回
        x * 10000
    }; // 这里以分号结束，将if表达式中的值绑定到y上
    println!("y = {}", y);

}