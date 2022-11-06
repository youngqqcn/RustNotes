// Author: yqq
// Date: 2022-11-06 15:46:25
// Description: match匹配



fn main() {

    let n = 13;

    match n {
        1 => println!("one"),
        2 | 3 | 4 | 5 | 6 | 7 => println!("goooood"),
        // 13..19 => println!("nnnnn"), // 暂不支持
        _ => println!("other"),
    }

    let b = true;

    // match 分支必须覆盖所有可能的值
    let ret = match b {
        true => "1",
        false => "0"
    };

    println!("{}", ret);
}