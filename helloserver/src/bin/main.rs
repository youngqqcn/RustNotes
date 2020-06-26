use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;

use helloserver::ThreadPool;

fn main() {

    // let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let listener = TcpListener::bind("0.0.0.0:58888").unwrap();

    // one connection per thread
    /*
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        thread::spawn(|| {
            println!("create new handler thread ");
            handle_connection(stream);
        });
    }
    */

    let thread_pool = ThreadPool::new(10);

    for stream in listener.incoming().take(10) {
        let stream = stream.unwrap();
        println!("Connection established!");
        thread_pool.execute(|| {
            handle_connection(stream);
        });

    }

    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    println!("Request msg: {}", String::from_utf8_lossy(&buf[..]));

    let get = b"GET / HTTP/1.1\r\n"; // b"" 字节字符串语法将其转换为字节字符串
    let sleep = b"GET /sleep HTTP/1.1\r\n"; 

    /*
    for i in 0..sleep.len() {
        print!("{}", &sleep[i]);
    }
    println!("");

    println!("{:?}", &buf);
    for i in 0..buf.len() {
        print!("{}", &buf[i]);
    }
    println!("");
    */

    let (status_line, filename) = if buf.starts_with(get) {
        println!("============1===============");
        ("HTTP/1.1 200 OK\r\n\r\n{}", "test.html")
    } else if buf.starts_with(sleep) {
        println!("============2===============");
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n{}", "test.html")
    } else {
        println!("============4===============");
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    



    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}