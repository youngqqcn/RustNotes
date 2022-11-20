// Author: yqq
// Date: 2022-11-20 11:12:02
// Description:


use std::num::ParseIntError;

type MyType<T> = Result<T, ParseIntError> ;

fn multiply_v2(first_number_str: &str, second_number_str: &str) -> MyType<i32> {
    first_number_str.parse::<i32>().and_then( | first_number | {
        second_number_str.parse::<i32>().map(|second_number|{
            first_number * second_number
        })
    })
}


fn multiply_v3(first_number_str: &str, second_number_str: &str) -> MyType<i32> {
    first_number_str.parse::<i32>().and_then( | first_number | {
        second_number_str.parse::<i32>().and_then(|second_number|{
            Ok(first_number * second_number)
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

    // let twenty = multiply_v1("10", "22");
    // println!("{}", twenty);
    // print(twenty);

    let t = multiply_v2("10", "220");
    print(t);
}