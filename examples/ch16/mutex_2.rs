
use std::sync::Mutex;
use std::rc::Rc;
use std::thread;

fn main() {

    let counter = Rc::new( Mutex::new(0) );  // 在前面学习 Rc<T> 的时候, Rc<T> 只能用于单线程
    let mut handles = vec![];

    for _ in 0..10 {
        let ctr = Rc::clone(&counter);
        let handle = thread::spawn(move || {  //`std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
            let mut num = ctr.lock().unwrap();   // the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}