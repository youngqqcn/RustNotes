// Author: yqq
// Date: 2022-11-07 22:43:56
// Description: 闭包作为函数输入参数

/*

Fn：表示捕获方式为通过引用（&T）的闭包
FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
FnOnce：表示捕获方式为通过值（T）的闭包



在满足使用需求的情况下， 编译器会使用权限最小的？

*/

// 该函数将闭包作为参数并调用它。
fn apply<F>(f: F) where F: FnOnce() { // 闭包没有输入值和返回值。
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。
    f();
}


// 输入闭包，返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32 where
    // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: FnMut(i32) -> i32 {
        // Fn 是最低要求， 使用FnOnce当然也可以运行, 但是使用 FnMut则不行

    f(3)
}



fn main() {

    use std::mem;

    let greeting = "hello";

    let mut vv =  "goodbye".to_owned();


    let makechange = || {

        println!("{}", greeting);

        vv.push_str("!!!!");
        println!("Then I screamed {}.", vv);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `vv`。
        // 现在需要 `FnOnce`。
        mem::drop(vv);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(makechange);




     // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
     let double = |x| 2 * x;

     println!("3 doubled: {}", apply_to_3(double));

}