// Author: yqq
// Date: 2022-11-13 09:49:40
// Description: 部分移动

/*
在单个变量的解构内，可以同时使用 by-move 和 by-reference 模式绑定。
这样做将导致变量的部分移动（partial move），这意味着变量的某些部分将被移动，
而其他部分将保留。在这种情况下，后面不能整体使用父级变量
，但是仍然可以使用只引用（而不移动）的部分。


*/

use std::fmt::Debug;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}


fn main() {

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // name 是 移动； age 是引用
    let Person {name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);


    // println!("Person: {:?}", person);
    // ^error: value borrowed here after partial move

    // person的name被移动了，部分移动，但是age还是可以继续用的
    println!("The person's age from person struct is {}", person.age);
}