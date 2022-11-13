// Author: yqq
// Date: 2022-11-13 16:10:02
// Description: trait 生命周期


#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32
}

impl <'a>Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10
        }
    }
}


fn main() {
    let b: Borrowed = Default::default();

    println!("b is {:?}", b);
}