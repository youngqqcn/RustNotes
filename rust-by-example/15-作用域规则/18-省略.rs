// Author: yqq
// Date: 2022-11-13 16:40:22
// Description: 生命周期的省略

/*
生命周期省略的省略规则：

1、每个引用参数都有不同生命周期参数
2、如果只有一个输入生命周期参数，就赋给输出生命周期参数
3、如果是方法，则将self的生命周期参数赋给所有输出参数

*/


fn single_input(x: &i32) {
    println!("{}", x);
}

fn aa_input<'a>(x: &'a i32) {
    println!("{}", x);
}

fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}


fn main() {
    let x = 3;

    single_input(&x);
    aa_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}