use std::sync::mpsc; // multiple producer , single consumer 
use std::thread;
use std::time::Duration;


fn main() {

    //mpsc::channel 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。
    let (tx, rx) = mpsc::channel();


    thread::spawn(move ||{
        let val = String::from("hello this is good name");
        println!("send msg:{}", &val);

        //send 方法返回一个 Result<T, E> 类型，
        //所以如果接收端已经被丢弃了，将没有发送值的目标，
        //所以发送操作会返回错误。
        tx.send(val).unwrap(); //出错的时候调用 unwrap 产生 panic
        // tx.send(val); //如果不处理 Result<T, E> 则编译时会报警告, 
                        // this `Result` may be an `Err` variant, which should be handled
    });

    let received = rx.recv().unwrap(); // recv() 会阻塞, 直到从通道接收到一个值,  recv() 返回 Result<T, E>
    println!("got msg:{}", &received);


    /*
    for i in 1..10 {
        let received = rx.try_recv().unwrap(); //非阻塞
        thread::sleep(Duration::from_secs(1));
        //做其他的事情
    }
    */


}