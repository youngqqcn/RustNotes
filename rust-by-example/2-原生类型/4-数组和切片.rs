// Author: yqq
// Date: 2022-11-03 23:17:57
// Description: 数组和切片

use std::mem;

// 数组：  某一种类型 T ， 内存连续存储， 大小在编译时确定， 记作 [T; length]

// 数组和切片的关系：
//      数组自动借用为切片
//      切片是数组一部分的引用

fn analyze_slice(s: &[i32]) {
    println!("first element of the slice: {}", s[0]);
    println!("the slice: {}", s.len());
}

fn main() {

    let xs = [1, 2, 3, 4]; // let xs: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", xs);


    let ys = [0; 500];  // 初始化500个0的数组
    println!("{:?}", ys[499]);

    // 数组是在栈上分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));


    // 数组可以
    analyze_slice(&xs);


    // 切片可以引用数组一部分
    analyze_slice(&ys[ 50 .. 60]);

    // 数组越界， 编译可以通过， 运行时崩溃
    println!("{}", xs[5]);
}