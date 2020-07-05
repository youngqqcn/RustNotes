use std::sync::{Mutex, Arc};


fn main() {

    let mtx_a = Arc::new( Mutex::new(0) );
    let mtx_b = Arc::new( Mutex::new(0) );

    let mtx_a_cp = mtx_a.clone();
    let mtx_b_cp = mtx_b.clone();
    let thd1 = std::thread::spawn(move || {
        for _ in 0..(1<<15) {
            let mut guard_b = mtx_b_cp.lock().unwrap();
            let mut guard_a = mtx_a_cp.lock().unwrap();
            *guard_a += 1;
            *guard_b += 1;
            println!("threadA: a={:?}", guard_a);
            println!("threadA: b={:?}", guard_b);
        }
        
    });

    let thd2 = std::thread::spawn(move || {
        for _ in 0..(1<<15) {
            let mut guard_a = mtx_a.lock().unwrap();
            let mut guard_b = mtx_b.lock().unwrap();
            *guard_a += 1;
            *guard_b += 1;
            println!("threadB: a={:?}", guard_a);
            println!("threadB: b={:?}", guard_b);
        }
    });

    println!("waitting for thread finished");
    thd1.join().unwrap();
    thd2.join().unwrap();
}