use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} for the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    }); // thread::spawn 的返回值类型是 JoinHandle。
        //JoinHandle 是一个拥有所有权的值,当对其调用 join 方法时，它会等待其线程结束。

    // handle.join().unwrap(); //阻塞等待子线程结束

    for i in 1..5 {
        println!("hi number {} for main thread!", i);
        thread::sleep(Duration::from_secs(1));
    }

    handle.join().unwrap(); //阻塞等待子线程结束
}