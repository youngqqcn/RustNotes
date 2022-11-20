// Author: yqq
// Date: 2022-11-20 10:50:14
// Description:

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let f = first_number_str.parse::<i32>().unwrap();
    let s = second_number_str.parse::<i32>().unwrap();

    f * s
}

fn main() {

    let t = multiply("10", "2");

    println!("{:?}", t);


    let xx = multiply("xxx", "xxx");
    println!("{:?}", xx);
}

