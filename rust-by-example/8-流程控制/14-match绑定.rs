// Author: yqq
// Date: 2022-11-06 18:32:48
// Description: match 绑定


fn age() -> u8 {
    18
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {


    // 如果不绑定， 只能知道范围， 而不能获取具体的值
    match age() {
        n @ 0..=3 =>  println!("baby, {} years old", n),
        n @ 4..=12 => println!("child, {} years old", n),
        n @ 14..=18 => println!("teenager, {} years old", n),
        n => println!("adult, {} years old", n)
    }

    match some_number() {
        // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
        Some(n @ 42) => println!("1==The Answer: {}!", n),
        Some(n @ 1..=100) => println!("2==The Answer: {}!", n),
        // 匹配任意其他数字。
        Some(n)      => println!("Not interesting... {}", n),
        // 匹配任意其他值（`None` 可变类型）。
        _            => (),
    }
}

