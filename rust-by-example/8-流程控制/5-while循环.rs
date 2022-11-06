// Author: yqq
// Date: 2022-11-06 12:11:51
// Description:

// https://leetcode.cn/problems/fizz-buzz/


// 给你一个整数 n ，找出从 1 到 n 各个整数的 Fizz Buzz 表示，并用字符串数组 answer（下标从 1 开始）返回结果，其中：

// answer[i] == "FizzBuzz" 如果 i 同时是 3 和 5 的倍数。
// answer[i] == "Fizz" 如果 i 是 3 的倍数。
// answer[i] == "Buzz" 如果 i 是 5 的倍数。
// answer[i] == i （以字符串形式）如果上述条件全不满足。
//  

// 示例 1：

// 输入：n = 3
// 输出：["1","2","Fizz"]
// 示例 2：

// 输入：n = 5
// 输出：["1","2","Fizz","4","Buzz"]
// 示例 3：

// 输入：n = 15
// 输出：["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/fizz-buzz
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use std::io;

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut values = Vec::<String>::new();

        let mut i = 1;
        while i <= n {
            if i % 3 == 0 && i % 5 == 0 {
                values.push(String::from("FizzBuzz") );
            } else if i % 3 == 0 {
                values.push(String::from("Fizz"));
            } else if i % 5 == 0 {
                values.push(String::from("Buzz"));
            } else {
                values.push( format!("{}", i));
            }
            i += 1;
        }

        return values;
    }
}

fn main() {

    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // let n = input.as_str().trim().parse::<u32>().unwrap();

    // let mut i = 1;
    // while i <= n {
    //     if i > 1 {
    //         print!(",");
    //     }

    //     if (i % 3 == 0) && (i % 5 == 0) {
    //         print!("FizzBuzz");
    //     } else if i % 3 == 0 {
    //         print!("Fizz");
    //     } else if i % 5 == 0 {
    //         print!("Buzz");
    //     } else {
    //         print!("{}", i);
    //     }
    //     i += 1;
    // }
    // println!("");


    println!("{:?}", Solution::fizz_buzz(15) );
}