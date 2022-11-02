// Author: yqq
// Date: 2022-11-02 22:52:15
// Description: 使用 ? 处理write错误



use std::fmt;


// 元组
struct MyList(Vec<i32>);

impl fmt::Display for MyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // vec引用
        let vs = &self.0;


        // 打印[
        write!(f, "[")?;

        // 遍历vec
        for (c, v) in vs.iter().enumerate() {
            if c > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", v)?; // 打印值
        }

        write!(f, "]")
    }
}



fn main() {
    println!("{}", MyList(vec![1, 2, 3, 4]));
}