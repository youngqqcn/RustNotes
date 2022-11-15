// Author: yqq
// Date: 2022-11-15 22:07:09
// Description: Clone

// 当处理资源时，默认的行为是在赋值或函数调用的同时将它们转移。
// 但是我们有时候也需要 把资源复制一份。
// Clone trait 正好帮助我们完成这任务。

#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {

    let nil = Nil;
    let copied_nil = nil;

    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair; // 如果没有实现 Copy trait, 则是 move
    println!("copy: {:?}", moved_pair);

    // move occurs because `pair` has type `Pair`,
    // which does not implement the `Copy` trait
    // println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone(); // 克隆
    // drop(moved_pair);
    println!("copy: {:?}", moved_pair);

    println!("clone: {:?}", cloned_pair);
}