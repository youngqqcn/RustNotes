// enum List { //recursive type has infinite size
//     Cons(i32, List),  //recursive without indirection
//     Nil,
// }

use std::fmt;

#[derive(Debug)]
enum List { //recursive type has infinite size
    Cons(i32, Box<List>),  //recursive without indirection
    Nil,
}


impl fmt::Display for List {

    fn fmt(&self, f: &mut fmt::Formatter<'_>)  -> fmt::Result {
        write!(f, "{}", self)
    }
}

use List::{Cons, Nil};


fn main() {
    let lst = Cons(1, 
        Box::new(Cons(2, 
                Box::new(Cons(3, 
                    Box::new(Nil))))));


    println!("{:?}", lst);


    let mut ps = Box::new(String::new());
    ps.push_str("dsfsdfsdfs");
    println!("{}", ps);



    let x = 5;
    let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(, x);
    println!("x = {}", x);
    println!("y = {}", y);

    // assert_eq!(5,  y);  //no implementation for `{integer} == &{integer}`
    assert_eq!(5,  *y);
    println!("y = {}", *y);



    let tmp_x = 5;
    let y  = Box::new(tmp_x);

    assert_eq!(5, x);
    assert_eq!(5, y); // no implementation for `{integer} == std::boxed::Box<{integer}>`
    assert_eq!(5, *y);

}