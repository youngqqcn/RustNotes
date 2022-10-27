
use std::thread;

fn main() {

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { //使用 move 关键字强制获取它使用的值的所有权,  移动了所有权
        println!("Here's a vector: {:?}", v);
    });

    // 违反了借用规则,编译报错! 在编译时期避免了潜在bug
    // drop(v); //value used here after move

    handle.join().unwrap();

}