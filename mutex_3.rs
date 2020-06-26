use std::sync::{Mutex, Arc};
// use std::rc::Rc;
use std::thread;
use std::time::Duration;

fn main() {

    let counter = Arc::new( Mutex::new(0) ); 
    let mut handles = vec![];


    for i in 0..10 {
        let ctr = Arc::clone(&counter);  // 


        // 不是交叉执行???
        let handle = thread::spawn(move || { 
            let mut num = ctr.lock().unwrap();//此处获取了锁, 其他线程只能等待
            for _ in 0..3 {
                // let mut num = ctr.lock().unwrap();//此处获取了锁, 其他线程只能等待
                println!("thread {} +1", &i);
                *num += 1;
                thread::sleep(Duration::from_secs(1));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


/*   为什么不是交叉执行??

thread 1 +1
thread 1 +1
thread 1 +1
thread 3 +1
thread 3 +1
thread 3 +1
thread 6 +1
thread 6 +1
thread 6 +1
thread 9 +1
thread 9 +1
thread 9 +1
thread 4 +1
thread 4 +1
thread 4 +1
thread 7 +1
thread 7 +1
thread 7 +1
thread 5 +1
thread 5 +1
thread 5 +1
thread 0 +1
thread 0 +1
thread 0 +1
thread 8 +1
thread 8 +1
thread 8 +1
thread 2 +1
thread 2 +1
thread 2 +1
Result: 30


*/