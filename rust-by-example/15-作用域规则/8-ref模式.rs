// Author: yqq
// Date: 2022-11-13 12:37:59
// Description: ref模式

// ref 可以在解构是 拿到引用 ， 使用 & 则不行



#[derive(Clone, Copy)]
struct Point {x: i32, y: i32 }


fn main() {

    let c = 'Q';


    let ref r_c1 = c;
    // 这2种等效
    let r_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *r_c1 == *r_c2);


    let point = Point { x: 0, y: 0 };


    // 在解构一个结构体时 `ref` 同样有效。
    let _copy_of_x = {
        // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
        let Point { x: ref ref_to_x, y: _ } = point;

        // 解构时，不能用 & 拿到引用
        // let Point { x: &ref_to_x, y: _ } = point; // error

        // 返回一个 `point` 的 `x` 字段的拷贝。
        *ref_to_x
    };


    // `point` 的可变拷贝
    let mut mutable_point = point;

    // `point` 的可变拷贝
    let mut mutable_point = point;

    {
        // `ref` 可以与 `mut` 结合以创建可变引用。
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // 通过可变引用来改变 `mutable_point` 的字段 `y`。
        *mut_ref_to_y = 1;
    }


    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);


    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // 解构 `mutable_tuple` 来改变 `last` 的值。
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}