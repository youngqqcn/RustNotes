
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


use crate::List::*;

fn main() {

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(100)), Rc::clone(&a));


    *value.borrow_mut() += 10;
    
    println!("a = {:?}", a);
    println!("a = {:?}", b);
    println!("a = {:?}", c);

}