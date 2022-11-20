// Author: yqq
// Date: 2022-11-20 10:57:57
// Description:

use std::num::ParseIntError;

fn multiply_v1(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {

    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}


fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then( | first_number | {
        second_number_str.parse::<i32>().map(|second_number|{
            first_number * second_number
        })
    })
}



fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error: {}", e),
    }
}




fn main() {

    let twenty = multiply_v1("10", "22");
    // println!("{}", twenty);
    print(twenty);

    let t = multiply_v2("10", "220");
    print(t);
}