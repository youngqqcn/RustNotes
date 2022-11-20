// Author: yqq
// Date: 2022-11-20 15:52:03
// Description:
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
struct DoubleError;

type MyResult<T> = std::result::Result<T, DoubleError>;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> MyResult<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}


fn print(result: MyResult<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn main() {

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
