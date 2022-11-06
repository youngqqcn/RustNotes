// Author: yqq
// Date: 2022-11-06 17:08:08
// Description:

enum Color {
    // 这三个取值仅由它们的名字（而非类型）来指定。
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    // println!("hello world");
    let color = Color::HSV(10, 20, 30);

    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        // Color::HSL(h, s, l) =>
        //     println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        // Color::CMY(c, m, y) =>
        //     println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        // Color::CMYK(c, m, y, k) =>
        //     println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
        //         c, m, y, k),
        _ => println!("other"),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}