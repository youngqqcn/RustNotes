// Author: yqq
// Date: 2022-11-06 12:46:56
// Description: for循环


struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut values = Vec::<String>::new();

        // let mut i = 1;
        for i in 1..n+1 {
            if i % 3 == 0 && i % 5 == 0 {
                values.push(String::from("FizzBuzz") );
            } else if i % 3 == 0 {
                values.push(String::from("Fizz"));
            } else if i % 5 == 0 {
                values.push(String::from("Buzz"));
            } else {
                values.push( format!("{}", i));
            }
        }

        return values;
    }
}



fn main() {
   
    println!("{:?}", Solution::fizz_buzz(15) );
}