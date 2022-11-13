// Author: yqq
// Date: 2022-11-13 15:39:51
// Description: 函数和生命周期

/*
排除省略（elision）的情况，带上生命周期的函数签名有一些限制：
- 任何引用都必须拥有标注好的生命周期。
- 任何被返回的引用都必须有和某个输入量相同的生命周期或是静态类型（static）。
*/


fn print_one(x: &i32) {
    println!("===> x {}", x);
}

fn print_one_ex<'a>(x: &'a i32) {
    println!("===> x {}", x);
}

fn add_one(x: &mut i32) {
    *x += 1;
}


fn add_one_ex<'a>(x: &'a mut i32) {
    *x += 1;
}


fn print_multi(x: &i32, y: &i32) {
    println!("xxxxxx => x is {}, y is {}", x, y);
}


fn print_multi_ex<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("xxxxxx => x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}

fn pass_x_ex<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    x
}


fn main() {

    let x = 1;
    print_one(&x);
    print_one_ex(&x);


    let mut y = 3;
    add_one(&mut y);
    print_one(&y);
    add_one_ex(&mut y);
    print_one(&y);

    print_multi(&x, &y);
    print_multi_ex(&x, &y);

    let z = pass_x(&x, &y );
    print_one_ex(z);
    
    let zz = pass_x_ex(&x, &y );
    print_one_ex(zz);
}