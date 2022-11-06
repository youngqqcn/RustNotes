// Author: yqq
// Date: 2022-11-06 15:58:45
// Description: match结构元组



fn main() {
    let tp = (1, 2, 3);

    println!("{:?}", tp);


    match tp {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        _      => println!("It doesn't matter what they are"),
    }
}