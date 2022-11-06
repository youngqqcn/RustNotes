// Author: yqq
// Date: 2022-11-06 09:33:13
// Description: 类型推断






fn main() {

    let e = 5u8;

    // 不知道存什么类型
    let mut v = Vec::new();

    // 到此可以推导出存放 u8 类型
    v.push(e);

    println!("{:?}", v);



    let mut vv = Vec::new();

    // 存放的是 单元类型（unit type）
    vv.push(());

    println!("{:?}", vv);

}