use std::thread;
use std::sync::{RwLock, Arc};
use std::time::Duration;


fn main() {

    let novel = Arc::new(RwLock::new(String::new()));

    let novel_w = novel.clone();
    let writer = std::thread::spawn(move || {
        let setenses = vec![
            "It was the best of times, ",
            "it was the worst of times, ",
            "it was the age of wisdom, ",
            "it was the age of foolishness, ",
            "it was the epoch of belief, ",
            "it was the epoch of incredulity, ",
            "it was the season of Light, ",
            "it was the season of Darkness, ",
            "it was the spring of hope, ",
            "it was the winter of despair, ",
            "we had everything before us, ",
            "we had nothing before us,",
        ];

        //写数据
        for s in setenses {
            {
                let mut nov = novel_w.write().unwrap();
                println!("writer writting a setense");
                *nov = String::from(s);
            }
            
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mut readers = vec![];

    //创建是个线程
    for i  in 0..10 {
        let nov_r = novel.clone();
        readers.push(std::thread::spawn(move || {
            //读数据
            for _ in 0..10{
                {
                    let nov = nov_r.read().unwrap();
                    println!("reader{:?}: {:?}", i, *nov);
                }
                thread::sleep(Duration::from_secs(1));
            }
        }));
    }


    writer.join().expect("writer join error");
    for r in readers {
        r.join().expect("reader join error");
    }

}