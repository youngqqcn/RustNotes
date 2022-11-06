// Author: yqq
// Date: 2022-11-06 18:19:00
// Description: 解构 结构体


struct Foo {
    x: (i32, i32),
    y: u32
}

fn main() {

    let foo = Foo {x: (12, -34), y:30};


    let Foo { x: m, y: n } = foo;
    println!("m={:?}, n={:?}", m, n);


    let Foo { y: k , ..} = foo;
    println!("k = {}", k);

}