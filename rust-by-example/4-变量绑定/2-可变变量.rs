// Author: yqq
// Date: 2022-11-05 22:49:32
// Description: 可变变量


// 变量绑定默认是不可变的， 如需改变，则需要加上 mut



fn main() {

    let a = 1;
    // a = 2; // error

    let mut b = 2;
    println!("b {}", b);
    b = 3; // ok
    println!("b {}", b);
}