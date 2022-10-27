use std::sync::{
    Mutex, Arc
};

use std::rc::Rc;

/*
fn foo1() {
    let counter_mtx = Mutex::new(0);

    let mut threads = Vec::new();;

    for i in 0..10 {
        let hd = std::thread::spawn( move || {
            let mut num = counter_mtx.lock().expect("lock error");
            *num += 1;
        });

        threads.push(hd);
    }

    for thd in threads {
        thd.join().expect("join error");
    }

    println!("{:?}",  *counter_mtx.lock().expect("lock error"));
}
*/

/*
// `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
fn foo2() {
    let counter_rc = Rc::new( Mutex::new(0) );

    let mut threads = Vec::new();;

    for i in 0..10 {
        let cnt = counter_rc.clone();
        let hd = std::thread::spawn( move || {
            let mut num = cnt.lock().expect("lock error");
            *num += 1;
        });

        threads.push(hd);
    }

    for thd in threads {
        thd.join().expect("join error");
    }

    println!("{:?}",  *counter_rc.lock().expect("lock error"));
}
*/

fn foo3() {
    let counter_arc = Arc::new( Mutex::new(0) );

    let mut threads = Vec::new();

    for _ in 0..10 {
        let cnt = counter_arc.clone();
        let hd = std::thread::spawn( move || {
            let mut num = cnt.lock().expect("lock error");

            // pub fn lock(&self) -> LockResult<MutexGuard<T>> 
            // lock() 返回的是 MutexGuard ,离开作用域会自动 unlock
            // 需要解引用才能获取到 T
            *num += 1;
        });

        threads.push(hd);
    }

    for thd in threads {
        thd.join().expect("join error");
    }

    println!("{:?}",  *counter_arc.lock().expect("lock error"));
}

fn main() {

    // foo1();
    // foo2();
    foo3();
   
}