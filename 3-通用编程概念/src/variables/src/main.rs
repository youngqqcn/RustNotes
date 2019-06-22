

/*
变量默认是不可变的 (immutable)


*/


/*
fn main() {
    let x = 5;
    x = 5;
    println!("Hello, world!");
}
*/


/*
fn main(){

    let x;// = 5;
    x = 6;
    //x = 9; // error
    println!("{}", x)

}
*/


/*

//常量  const 修饰
// 不能改变值
// 不能使用 mut 修饰
fn main(){

    //Rust 常量的命名规范是使用下划线分隔的大写字母单词
    //并且可以在数字字面值中插入下划线来提升可读性
    const MAX_POINTS : u32 = 1_000_000;
    println!("{}", MAX_POINTS)
}
*/




// 隐藏
fn main(){


    let x = 5;

    let x = x + 999;  // 第一个  x 被隐藏

    let x = x * 2;  

    println!("x = {}", x);


    // let mut str_sum = "";
    // str_sum = str_sum.len(); // error

    let str_sum  = "你好";
    let str_sum = str_sum.len(); //ok  , 新建了一个新变量(usize类型)
    println!("len: {}", str_sum);

}



