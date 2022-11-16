// Author: yqq
// Date: 2022-11-16 22:28:57
// Description: 重复

// 宏在参数列表中可以使用 + 来表示一个参数可能出现一次或多次，
// 使用 * 来表示该 参数可能出现零次或多次。


macro_rules! find_min {

    // 递归结束
    ($x: expr) => ($x);

    ($x: expr, $($y: expr), +) => (
                          // 递归
        std::cmp::min($x, find_min!($($y),+))
    )
}


fn main() {

    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(1u32, 2u32 * 3, 4u32));
}