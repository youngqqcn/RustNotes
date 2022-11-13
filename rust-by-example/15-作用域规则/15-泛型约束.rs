// Author: yqq
// Date: 2022-11-13 16:15:25
// Description: 生命周期 泛型约束


use std::fmt::Debug;

// struct Ref<'a, T: 'a>(&'a T);

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T> (t: T)  where T: Debug {
    println!("print t is {:?}", t);
}

fn print_ref<'a , T>(t: &'a T) where T: Debug + 'a {
    println!("print_ref t is {:?}", t);
}

fn main() {
    let x = 7;
    let refx = Ref(&x);

    print(x);
    print(&refx);
    print(refx);

    //
    print_ref(&x);
}