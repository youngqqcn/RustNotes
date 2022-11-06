// Author: yqq
// Date: 2022-11-06 10:41:17
// Description: ToString 和 FromStr

use std::fmt;
use std::string::ToString;

struct Square {
    x: i32
}

// 一般通过实现 fmt::Display ， 会自动获得 ToString
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "x:{}", self.x)
    }
}

// 不能同时实现 fmt::Display 和 ToString, 否则会报冲突的错误
// conflicting implementations of trait `std::string::ToString` for type `Square`
// // 直接使用 ToString
// impl ToString for Square {
//     fn to_string(&self) -> String {
//         format!("x: {:?}", self.x)
//     }
// }



fn main() {

    let s = Square{x: 9};
    println!("{}", s);
    println!("{}", s.to_string());
}