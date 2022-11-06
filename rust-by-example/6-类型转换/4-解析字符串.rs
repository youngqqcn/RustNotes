// Author: yqq
// Date: 2022-11-06 10:56:26
// Description: 解析字符


fn main() {

    let n: i32 = "444".parse::<i32>().unwrap();
    println!("n = {}", n);


    let f: f32 = "34.234232".parse().unwrap();
    println!("f = {}", f);


    // 尝试解析错的
    let x = match "sfd233".parse::<i32>() {
        Ok(v) => v,
        Err(err) => {
            println!("err: {}", err);
            0
        }
    };
    println!("x = {}", x);


    // 会panic
    let y =  "sfd233".parse::<i32>().unwrap();
    println!("y = {}", y);
}