use std::thread;
use std::sync::mpsc::{
    sync_channel, 
    SyncSender, Receiver
};
use std::time::Duration;

fn foo1() {

    // 同步通道
    //设置buffer 为 2,  如果buffer大小为0, 
    //则表示发送方send方法会一致阻塞知道接收方消费
    // let (tx, rx) = sync_channel(0);  
    let (tx, rx) = sync_channel(1);   //当队列满了之后, 会阻塞到队列再一次开放

    std::thread::spawn(move || {
        let vcts = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for item in vcts {
            tx.send(item).expect("send error");
            println!("sent ok");
        }
    });


    for rev in rx {
        println!("got: {:?}", rev);
        thread::sleep(Duration::from_secs(5));
    }

}


fn main() {
    foo1();
}