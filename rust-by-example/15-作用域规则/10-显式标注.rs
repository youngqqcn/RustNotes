// Author: yqq
// Date: 2022-11-13 15:30:23
// Description:  显式标注


fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {}, y is {}", x, y);
}

// 'a
fn failed_borrow<'a>() {
    let _x = 12;

    // let y: &'a i32 = &_x;

}// 到这里 _x 就释放了


// 'a
fn failed_borrow_ex<'a>(x: &'a i32) {
    println!("x is {}", x);
}// 到这里 _x 就释放了


fn main() {

    let (f, n) = (4, 9);
    print_refs(&f, &n);

    // `failed_borrow` 未包含引用，因此不要求 `'a` 长于函数的生命周期，
    // 但 `'a` 寿命确实更长。因为该生命周期从未被约束，所以默认为 `'static`。
    failed_borrow();

    let x = 99;
    failed_borrow_ex(&x);
}