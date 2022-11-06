// Author: yqq
// Date: 2022-11-06 12:49:02
// Description: for 与 迭代器

// for与集合的迭代器配合
//    iter()  只读迭代器
//    into_iter()   消耗迭代器
//    iter_mut()   可写迭代器


fn main() {

    let mut v = vec![1, 2, 3, 4, 5];

    for it in v.iter() {
        println!("{}", it);
    }
    println!("{:?}", v);

    for it in v.iter_mut() {
        *it *= *it;
    }
    println!("{:?}", v);


    for it in v.into_iter() { // v 被move了， 后续就不能使用了
        println!("{}", it);
    }
    // println!("{:?}", v); // error


}