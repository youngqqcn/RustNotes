use std::sync::{
    Arc, 
    // RwLock, 
    Mutex, 
    // MutexGuard,
    // RwLockReadGuard, 
    // RwLockWriteGuard
};

use std::time::Duration;


fn main() {

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    // 创建11个线程
    for i in 0..=10 {
        // let cnt = counter.clone();
        let cnt = Arc::clone(&counter);
        handles.push( std::thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += i;
            std::thread::sleep(Duration::from_secs(1)); //每个线程等 1s  
        }));
    }

    //等待子线程退出 
    println!("waitting for thread exit");
    for handle in handles {
        handle.join().expect("join error");
    }

    println!("all finished");
    println!("counter is {:?}", *counter.lock().unwrap());
}