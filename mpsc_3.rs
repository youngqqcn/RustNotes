use std::thread;
use std::time::Duration;
use std::sync::mpsc;


// 多个生产者  单个消费者

fn main() {


    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    // 生产者1
    thread::spawn(move || {

        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];

        for val in vals {
            println!("thread 1 send {}", &val);
            tx1.send(val).unwrap();
            thread::sleep( Duration::from_secs(1) );
        }
        println!("thread 1 send finished");
    });


    // 生产者2
    thread::spawn(move || {
        let vals = vec! [
            String::from("apple"),
            String::from("bear"),
            String::from("cat"),
            String::from("dog"),
        ];

        for val in vals {
            println!("thread 2 send {}", &val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        println!("thread 2 send finished");
    });



    //主线程作为消费者(单一)
    for recv_msg in rx {
        println!("got: {}", recv_msg);
    }

    println!("main receive finished");

}