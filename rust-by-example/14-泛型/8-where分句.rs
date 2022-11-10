
// Author: yqq
// Date: 2022-11-10 22:37:37
// Description: where 分句

/*
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

使用 `where` 从句来表达约束

impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}

*/

use std::fmt::Debug;


trait PrintInOption {
    fn print_in_option(self);
}


impl<T> PrintInOption for T where
    Option<T>: Debug
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }

}

fn main() {
    let v = vec![1, 2, 3 ];
    v.print_in_option();

    let a = [1, 2 ,333 ];
    a.print_in_option();
}