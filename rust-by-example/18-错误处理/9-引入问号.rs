// Author: yqq
// Date: 2022-11-20 12:35:10
// Description: 引入?

use std::num::ParseIntError;


fn multiply(f_str: &str, s_str: &str) -> Result<i32, ParseIntError> {
    let f = f_str.parse::<i32>()?;
    let s = s_str.parse::<i32>()?;

    Ok(f * s)
}


// 在 ? 出现以前，相同的功能是使用 try! 宏完成的。
// 现在我们推荐使用 ? 运算符，但是 在老代码中仍然会看到 try!。
fn multiply_old(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = try!(first_number_str.parse::<i32>());
    let second_number = try!(second_number_str.parse::<i32>());

    Ok(first_number * second_number)
}

fn print(r: Result<i32, ParseIntError>) {
    match r {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("err: {}", e),
    }
}



fn main() {
    print(multiply("10", "2"));
    print(multiply("1x", "2"));
    print(multiply_old("10", "2"));
    print(multiply_old("1x", "2"));
}