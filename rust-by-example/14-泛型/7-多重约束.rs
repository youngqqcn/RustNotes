// Author: yqq
// Date: 2022-11-10 22:30:53
// Description: 多重约束
use std::fmt:: {
    Debug, Display
};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Display>(t: &T, u: &U ) {
    println!("t : `{:?}`", t);
    println!("u : `{}`", u);
}


fn main() {

    let s = "word";
    let a = [1, 2, 3];
    let v = vec![1, 2, 3];

    compare_prints(&s);

    compare_types(&v, &s);
}