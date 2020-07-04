use std::thread;
use std::sync::mpsc::{
    channel
};

use std::time::Duration;

fn foo1(){
    let (tx, rx) =  channel();

    thread::spawn(move||{
        let msg = String::from("goood");
        tx.send(msg).expect("send error");
    });

    let recv_msg = rx.recv().expect("receive error");
    println!("receive msg: {}", recv_msg);
}

fn foo2() {

    //异步通道,即发送方不需要等待,可以不停地向通道中发送数据
    // 因为存在 "infinite buffer" (无限buffer)
    let (tx, rx) = channel(); 
    let handle = std::thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
        ];

        //不阻塞, 一次性全部发完, 不必等待消费者消费
        for v in vals {
            println!("send {:?}", v);
            tx.send(v).expect("send error");
        }

        println!("sender thread is finished");
    });//发送线程结束, 离开作用域, tx将释放, channel关闭.

    for recv in rx { 
        //接收方阻塞等待消息到达, 此时发送方线程可能已经结束,
        //但是, buffer中还有消息, 继续读取, 知道buffer为空了之后, 
        //接收才会退出
        thread::sleep(Duration::from_secs(1));
        println!("got: {:?}", recv);
    }

    //阻塞等待子线程退出
    handle.join().expect("thread panicked");
}


// 多个发送方
fn foo3() {
    
    let (tx, rx) =  channel();

    let tx_copy = tx.clone();


    std::thread::spawn(move || {
        let vals = vec![
            String::from("A:1"),
            String::from("A:2"),
            String::from("A:3"),
        ];

        for val in vals { //move
            tx_copy.send(val).expect("send error");
            thread::sleep(Duration::from_secs(1));
        }
    });

    std::thread::spawn(move || {
        let vals = vec![
            String::from("B:1"),
            String::from("B:2"),
            String::from("B:3"),
        ];

        for val in vals { //move
            tx.send(val).expect("send error");
            thread::sleep(Duration::from_secs(1));
        }
    });

    //接收方
    for rev in rx {//move
        println!("{:?}", rev);
        thread::sleep(Duration::from_secs(3));
    }

}


fn main() {
    // foo1();
    // foo2();
    foo3();
}