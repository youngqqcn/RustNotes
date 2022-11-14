// Author: yqq
// Date: 2022-11-14 21:50:08
// Description: trait 派生

// 下面是可以自动派生的 trait：

// 比较 trait: Eq, PartialEq, Ord, PartialOrd
// Clone, 用来从 &T 创建副本 T。
// Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
// Hash，从 &T 计算哈希值（hash）。
// Default, 创建数据类型的一个空实例。
// Debug，使用 {:?} formatter 来格式化一个值。


#[derive(PartialEq, PartialOrd )]
struct Centimeters(f64);


struct CC(f64);



#[derive(Debug)]
struct Inches(f64);

impl Inches {

    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}


fn main() {
    // println!("hello world");

    let c1 = Centimeters(55.55);
    let c2 = Centimeters(55.55);

    let _ff = c1 == c2;

    let cc1 = CC(1.1);
    let cc2 = CC(1.1);

    // binary operation `==` cannot be applied to type `CC`
    // let _f = cc1 == cc2;


    let foot = Inches(333.32);
    println!("{:?}", foot);

    let cmp = if foot.to_centimeters() < c2 {
        "smaller"
    } else {
        "bigger"
    };

    println!("{}", cmp);

}