// Author: yqq
// Date: 2022-11-03 21:47:00
// Description: Rust原生类型


/*

标量类型：
    整数:
        有符号： i8, i16, i32, i64, i128, isize
        无符号： u8, u16, u32, u64, u128, usize

    浮点数：
        f32, f64

    布尔类型： bool

    字符: char      注意： 在Rust中 char 是 4个字节（Unicode）

    单元类型：  ()     空元组

复合类型：
    数组：
    元组：

默认值：
    浮点数如果不说明类型，默认为f64
    整型默认为：i32
 */


fn main() {

    let ok = true;
    let bad: bool = true;

    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;

    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;

}