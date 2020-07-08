use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};


// struct Job = 
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: i32,
    thread: Option< thread::JoinHandle<()>>
}

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}



impl Worker {
    fn new(id: i32,  arc_receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            
            loop {
                let msg = arc_receiver.lock().unwrap().recv().unwrap(); 
                match msg {
                    Message::NewJob(job) => {
                        // 使用 loop 并在循环块之内而不是之外获取锁和任务，
                        // lock 方法返回的 MutexGuard 在 let job 语句结束之后立刻就被丢弃了
                        // let job = arc_receiver.lock().unwrap().recv().unwrap();


                        println!("worker {} got a job, executing...", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        
                        break;
                    }
                }
            }

            /*
            // 因为 while 表达式中的值在整个块一直处于作用域中，
            // job() 调用的过程中其仍然持有锁，
            // 这意味着其他 worker 不能接收任务。

            // 这样做也是合理的, 反过来思考一下: 如果不持有MutexGuard, 那么 while循环中的代码将被别的线程干扰
            while let Ok(job) = arc_receiver.lock().unwrap().recv() {
                println!("worker {} got a job, executing...", id);

                job(); 
            }
            */

        });

        Worker{
            id,
            thread: Some(thread),
        }
    }
}


impl ThreadPool {
    /// 创建线程池。
    ///
    /// `size`: 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        let arc_receiver = Arc::new( Mutex::new(receiver) );

        for i in 0..size {
            // workers.push(Worker {id:i as i32 +1000, handle: thread::JoinHandle<()>);
            workers.push( Worker::new(i as i32, Arc::clone(&arc_receiver) ) )
        }


        ThreadPool {
            workers,
            sender,
        }

        
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();  //向channel中发送任务
    }
}


impl Drop for ThreadPool {

    /// 这里为什么使用两个循环,而不使用一个循环?
    /// 如果使用一个循环, 即发送了Terminate消息之后, join() 等待当前被迭代的worker退出
    /// 但是, 并不能保证 Terminate消息被当前Worker收到, 
    /// 如果 被其他Worker收到, 而当前迭代的worker没有收到, 那么就是一直等下去!  造成死锁
    /// 
    /// 所以, 分成两个循环, 一个循环单独发送消息 , 发送完了之后, 另一个循环 再join等待worker退出
    /// 这样就能保证,  在join前前, 每个worker都收到了 Terminate消息, 
    fn drop(&mut self) {

        // 发送退出消息
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");


        // 等待每个worker线程退出
        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            // 使用 if let 解构 Some 并得到线程，接着在线程上调用 join。
            // 如果 worker 的线程已然是 None，就知道此时这个 worker 已经清理了其线程所以无需做任何操作。
            // worker.thread.join().unwrap();
            if let Some(thread)  = worker.thread.take() {
                thread.join().unwrap();
            }

            // 这种方式存在问题, 即 如果一个线程已经是None, 再调用 join方法就会 panic
            // worker.thread.take().unwrap().join().unwrap();
        }

    }
}
