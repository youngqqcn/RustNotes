// Author: yqq
// Date: 2022-11-06 18:25:26
// Description: match 中使用if 卫语句



fn main() {

    // let p = (2, -5);
    // let p = (2, -2);
    let p = (1, 2);

    match p {
        (x, y) if x * y < 0 => println!("negative"),
        (x, y) if x * y >= 0 => println!("postive"),
        (x, y) if x + y == 0 => println!("absolute"),
        (x, _) if x % 2 != 0 => println!("odd first"),
        _ => println!("ok"),
    }
}