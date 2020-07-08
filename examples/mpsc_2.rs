use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {

    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("thread"),
        ];

        for val in vals {
            println!("send:{}", &val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        println!("tx  overed");
    });


    for recv in rx {   //tx发送方线程结束后, 被释放, 通道被关闭，迭代器也将结束。
        println!("got:{}", recv);
    }

    println!("rx overed");

}