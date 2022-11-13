// Author: yqq
// Date: 2022-11-13 09:35:34
// Description: 所有权和移动

// 因为变量要负责释放它们拥有的资源，
// 所以资源只能拥有一个所有者。这也防止了资源的重复释放。注意并非所有变量都拥有资源


// 在进行赋值（let x = y）或通过值来传递函数参数（foo(x)）的时候
// ，资源的所有权（ownership）会发生转移。按照 Rust 的说法，这被称为资源的移动（move）。
// 在移动资源之后，原来的所有者不能再被使用，这可避免悬挂指针（dangling pointer）的产生。





fn destroy_box(c: Box<i32>) {
    println!("destroying a box that contains: {}", c);

    // c 被释放
}



fn main() {

    // 栈上
    let x = 555u32;

    let y = x;

    println!("x is {}, y is {}", x, y);

    let a = Box::new(555i32);
    println!("a contains: {}", a);


    let b = a;

    destroy_box(b);

    // println!("b {}", b); // ERROR: value borrowed here after move

}