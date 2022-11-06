// Author: yqq
// Date: 2022-11-06 09:45:58
// Description: From 和 Into

use std::convert::From;



/*
trait 的定义, 其中 T 必须是具体类型


pub trait From<T> {
    fn from(T) -> Self;
}

*/


#[derive(Debug)]
struct MyNum {
    v: i32
}


impl MyNum {
    #[allow(dead_code)]
    fn new() -> MyNum {
        MyNum {
            v: 0
        }
    }
}

// 实现 use std::convert::From   trait
impl From<i32> for MyNum {
    fn from(v: i32) -> MyNum {
        MyNum {
            v: v
        }
    }

}

/*
pub trait Into<T> {
    fn into(self) -> T;
}

*/

impl Into<i32> for MyNum {
    fn into(self) -> i32 {
        self.v
    }
}


fn main() {
    let mystr = "hello";
    let mystring = String::from(mystr);
    println!("{}, {}", mystr, mystring);


    let n = MyNum::from(999i32);
    println!("{:?}", n);
    println!("{:?}", n.v);

    let x: MyNum = (9i32).into();
    println!("{:?}", x);

    let integer: i32 = x.into();
    println!("integer: {}", integer);

}