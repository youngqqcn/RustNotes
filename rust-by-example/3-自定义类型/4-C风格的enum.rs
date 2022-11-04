// Author: yqq
// Date: 2022-11-04 22:05:24
// Description: C风格的枚举类型

// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum XXX {
    X,
    Y,
    Z,
}


enum RGB {
    R = 0xff0000,
    G = 0x00ff00,
    B = 0x0000ff,
}


fn main() {
    // enum转为整型
    println!("X: {}, Y:{}, Z:{}", XXX::X as i32, XXX::Y as i32, XXX::Z as i32);

    // println!("green value #{:06x}", RGB::G );
    println!("green value #{:06x}", RGB::G as i32);
}