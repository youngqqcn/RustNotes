// Author: yqq
// Date: 2022-11-20 16:06:16
// Description:
use std::error;
use std::fmt;

// 为 `Box<error::Error>` 取别名。
type MyResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVecErr;

impl fmt::Display for EmptyVecErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVecErr {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> MyResult<i32> {
    vec.first()
        .ok_or_else(|| EmptyVecErr.into())
        .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| 2 * i))
}

fn print(result: MyResult<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
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
