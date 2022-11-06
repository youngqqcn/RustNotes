// Author: yqq
// Date: 2022-11-06 11:37:06
// Description: loop循环

// loop 表示一个无线循环， 可以使用  break 来退出

fn main() {

    let mut count = 0;
    let mut sum = 0;

    loop {
        // 循环结束
        if count >= 10000 {
            break
        }

        sum += count;
        count += 1;
    }

    println!("0 + 1 + 2 + ... + 10000 = {}", sum);

}