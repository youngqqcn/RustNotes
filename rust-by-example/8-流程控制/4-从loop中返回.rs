// Author: yqq
// Date: 2022-11-06 12:09:40
// Description: 从loop中返回


// 使用break 返回



fn main() {

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);


}