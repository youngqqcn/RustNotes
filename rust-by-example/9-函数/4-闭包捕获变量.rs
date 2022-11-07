// Author: yqq
// Date: 2022-11-07 22:14:04
// Description: 捕获


// 闭包可以通过以下3种方式捕获变量：
    // 通过引用：&T
    // 通过可变引用：&mut T
    // 通过值：T



fn main() {

    let color = String::from("red");

    let print = || println!("====> {}", color); // 这里是 &color

    print();

    let _reborrow = &color;
    print(); // ok

    let _moved = color;
    // print(); // error , 因为上面的color已经被移动


    //-------------------------------

    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // let _xxx = count; // 尝试移动count， 会导致后面的inc编译报错
    // let _xxx = &count; // 尝试借用， 但是后面的inc又会修改
    inc();

    let _xxx = &count; // 尝试借用， ok， 因为后面已经没有对count的可变借用了
    let _yyx = &mut count; // 尝试借用， ok， 因为后面已经没有对count的可变借用了
    // let _yy = &mut count; // error 不能同时有2个及以上可变引用
    *_yyx = 99;
    //-----------------------------------

    use std::mem;

    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);


    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable); // 已经被移动
    };
    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    // consume();

    //-------------------------------------


    let h = vec![1, 2, 3, 4];
    // let contains =   | v | h.contains(v);  // 没有move， 表示 h 没有移动
    let contains =  move | v | h.contains(v);  // move 表示 将 h 移动

    println!("{}", contains(&3));
    println!("{}", contains(&3));

    // println!("There're {} elements in vec", h.len());

}