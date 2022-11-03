// Author: yqq
// Date: 2022-11-03 22:58:54
// Description: 元组

use std::fmt;

// 元组可以做函数参数， 也可以用做函数返回值，返回多个值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    (pair.1, pair.0)
}



#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);


// 作业
impl fmt::Display for Matrix {
    fn fmt(&self, f :&mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}


fn transpose( m: &Matrix ) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}


fn main() {

    // 元组可以包含多种不同类型
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    // 通过下标访问元组元素
    println!("{:?}", long_tuple.0);
    println!("{:?}", long_tuple.5);

    // 元组也可以作为元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("{:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13); // 不能打印，因为默认Debug的实现，只支持最多12个值的元组
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);  // 可以打印
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。


    println!("one element tuple: {:?}", (5u32, )); // 两个元组
    println!("just an integer: {:?}", (5u32));



    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);


    //
    println!("{:?}", reverse((33, true)));



    let m = Matrix(1.1, 1.2, 1.3, 1.4);
    // println!("{:?}", m);
    println!("{}", m);

    println!("===========");
    println!("{}", transpose( &m ) );
}