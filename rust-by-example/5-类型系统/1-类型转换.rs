// Author: yqq
// Date: 2022-11-06 09:17:17
// Description:  类型转换


// Rust不支持隐式转换，但是可以用`as`关键字进行显式转换


#![allow(overflowing_literals)]

fn main() {
    let d =   234.23423423;  // 默认是f64

    // let x: u8 = d;  // error
    let x: u8 = d as u8;

    let ch = x as char;

    println!("==> {}, {}, {}", d, x, ch);


    // 整数截断
    println!("1000 as u16 : {}", 1000 as u16);
    println!("1000 as u8 : {}", 1000 as u8);
    println!("1000 % 256 : {}", 1000 % 256);
    println!("-1 as u8 : {}", (-1i8) as u8);

}