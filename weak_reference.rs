
use std::rc::{Rc, Weak};
use std::cell::RefCell;


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


fn main() {
    let  leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), //此处是空Weak引用
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); //upgrade获取不到时, 返回None
    println!("child: {:?}", leaf.children);

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 1, 0


    {
        let branch =  Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),  //将children 指向  leaf
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //将leaf的parent指向 branch, 

        
        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch)); //1, 1

        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); //2, 0

    } // branch离开作用于, 其children指向leaf的强引用自动减1,此时strong_count为0,故而将branch释放, 
      // leaf中parent指向branch的弱引用也自动减1, 此时为0; 因branch已释放, 所以leaf的strong_count也减1


    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // 此时 upgrade 返回 branch
    // println!("child: {:?}",  branch.children);

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); //1, 0

}