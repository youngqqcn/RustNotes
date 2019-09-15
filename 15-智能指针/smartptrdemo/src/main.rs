


/*
enum List{
    Cons(i32, Box< List >),
    Nil,
}


use List::{Cons, Nil};



fn main() {
//    println!("Hello, world!");


//    let b = Box::new(9);
//    println!("b = {}", b);



//    let list = Cons(1, Cons(2, Cons(3, Nil)));

    let list = Cons(1,
        Box::new(Cons(2,
        Box::new(Cons(3,
        Box::new(Nil)))))
        );



}
*/



/*
fn main(){

    let x = 5;
    let y = &x;


    assert_eq!(5, x);
//    assert_eq!(5, y); //can't compare `{integer}` with `&{integer}`
    assert_eq!(5, *y);
}


*/

/*
fn main(){

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
*/


use std::ops::Deref;


struct MyBox<T>(T);

impl<T>  MyBox<T>{
    fn new(x : T) -> MyBox<T> {
        MyBox(x)
    }
}


impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


fn hello(name : &str){
    println!("hello , {}", name);
}


fn main(){

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
//    assert_eq!(5, *(y.deref()));


    let m  = MyBox::new(String::from("Rust language"));
    hello( &m );
}



















