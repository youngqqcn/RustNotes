
use std::rc::{Rc, Weak};
use std::cell::RefCell;


#[derive(Debug)]
struct Node {
    value: i32,
    child: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>, // 指向父节点, 使用 Weak<T> 避免循环引用
}

fn main() {

    let leaf = Rc::new(
        Node{
            value:3, 
            child: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        }
    );

    println!("leaf parent = {:?}", leaf.parent.borrow());

    let branch = Rc::new(Node{
        value: 5,
        child: RefCell::new(vec![Rc::clone(&leaf)]),
    });

}