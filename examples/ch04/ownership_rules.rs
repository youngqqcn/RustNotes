

/*
fn main() {


    // move occurs because `s1` has type `std::string::String`, 
    // which does not implement the `Copy` trait
    let s1 = String::from("hello");
    let s2 = s1; // move 移动

    println!("s2:{}", s2);
    println!("{}, world!", s1); //借用 s1

}
*/



// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();  //克隆

//     println!("s2:{}", s2);
//     println!("{}, world!", s1); //借用 s1

// }


//原因是像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。
/*
 fn main() {
    let x = 5;
    let y = x;   //ok, 因为整型存储在栈上
    println!("x = {},  y = {}", x, y);
}
*/


//Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait
/*
所有整数类型，比如 u32。
布尔类型，bool，它的值是 true 和 false。
所有浮点数类型，比如 f64。
字符类型，char。
元组，当且仅当其包含的类型也都是 Copy 的时候。
比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
 */


 /*

 // 将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，
fn main() {

    let s = String::from("hello");

    takes_ownership(s); //move
    // println!("{}", s); //错误:  borrow of moved value: `s`

    let x = 5;
    make_copy(x);  // copy 这里是拷贝
    println!("{}", x); // ok

}

fn takes_ownership(some_string : String) {
    println!("{}", some_string);
}

fn make_copy(some_integer : i32) {
    println!("{}", some_integer);
}

*/


// 将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，
// 其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

// 返回值也可以转移所有权
fn main() {
    let s1 = geives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello"); 
    println!("{}", s2);

    let s3 = takes_and_gives_back(s2); // move in and move out

    // println!("{}", s2); //error 

    println!("{}", s3); //ok
}

fn geives_ownership() -> String {
    let some_str = String::from("hello world");
    some_str
}

fn takes_and_gives_back(s : String) -> String {
    s
}



