// Author: yqq
// Date: 2022-11-06 11:22:03
// Description: Rust的表达式



// 如果代码块的最后一个表达式结尾处有分号，则返回值为 ()


fn main() {

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };


    let z = {
        2 * x;   // 分号结尾， 返回 ()
    };
    println!("z = {:?}", z);


    println!("{:?}", y);


    let pp = {
        1 + 9   // 返回 10
    } * 10; // 10 * 10
    println!("pp = {}", pp);

    // println!("hello world");
}