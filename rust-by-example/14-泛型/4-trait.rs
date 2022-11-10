// Author: yqq
// Date: 2022-11-10 22:09:34
// Description: 泛型trait

use std::fmt;

struct MyType;


trait Fooo<T> {
    fn myshow(self, _: T) ;
}

impl <T, U> Fooo<T> for U
    where T : fmt::Debug
{
    fn myshow(self, t: T) {
        println!("{:?}", t);
    }
}


fn main() {
    MyType.myshow(333);
    MyType.myshow("hhhhhh");
}