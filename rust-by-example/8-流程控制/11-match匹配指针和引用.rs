// Author: yqq
// Date: 2022-11-06 17:11:38
// Description:


//  解引用： *
//  解构： &    ref    ref mut



fn main() {
    let r = &4;

    match r {
        &v => println!("1===={}", v), // v是i32类型
        v => println!("2===={}", *v), // v是&i32类型
        _ => println!("other"),
    }

    println!("------------------");
    match *r {
        v => println!("1===={}", v),
        _ => println!("other")
    }

    // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
    // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
    let _not_a_reference = 3;

    // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
    // 下面这行将得到一个引用。
    let ref _is_a_reference = 3;  // 等效于 let _is_a_reference = &3;


    // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字来创建引用。
    // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
    // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
    // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
    // 引用。
    match value {
        // ref r => println!("Got a reference to a value: {:?}", r),
        ref r => println!("Got a reference to a value: {:?}", *r),
    }


    match mut_value {
        ref mut rm => {
            *rm += 100;
            println!("{:?}", *rm);
        }
    }
}