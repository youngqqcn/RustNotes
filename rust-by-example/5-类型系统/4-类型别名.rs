// Author: yqq
// Date: 2022-11-06 09:39:27
// Description: 类型别名

// 类型  C语言中的 typedef  ，  C++中的  use  xx=T

type NanoSecond = u64;
type Inch = u64;


#[allow(non_camel_case_types)]
type u64_t = u64;


fn main() {

    let x: NanoSecond = 99999999999;
    println!("{}", x);

    // let y = 34323u64_t; // error
    let y = 34323 as u64_t; // ok

    println!("{}", y);
    let z = y as Inch;
    println!("{}", z);
}