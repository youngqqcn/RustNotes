// Author: yqq
// Date: 2022-11-13 09:59:34
// Description: 借用

// 多数情况下，我们更希望能访问数据，同时不取得其所有权。为实现这点，Rust 使用了借用（borrowing）机制。
// 对象可以通过引用（&T）来传递，从而取代通过值（T）来传递。

// 编译器（通过借用检查）静态地保证了引用总是指向有效的对象。
// 也就是说，当存在引用指向一个对象时，该对象不能被销毁

fn eat_box_i32( bi: Box<i32> ) {
    println!("destroying box that contains: {}", bi);
}

fn borrow_i32(bi: &i32) {
    println!("This int is : {}", bi);
}

fn main() {

    let bi = Box::new(5_i32);
    let si = 6_i32;


    borrow_i32(&bi);
    borrow_i32(&si);

    {
        let ri: &i32 = &bi;

        // eat_box_i32(bi); // 前面已经被借用，此处不能再被移动

        borrow_i32(ri);
    }

    eat_box_i32(bi);
}