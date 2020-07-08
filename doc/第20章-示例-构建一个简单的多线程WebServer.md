

## 第20章  构建多线程 web server


```rust

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
```


完善 webserver:

- 为 ThreadPool 和其公有方法增加更多文档
- 为库的功能增加测试
- 将 unwrap 调用改为更健壮的错误处理
- 使用 ThreadPool 进行其他不同于处理网络请求的任务
- 在 crates.io 上寻找一个线程池 crate 并使用它实现一个类似的 web server，将其 API 和鲁棒性与我们的实现做对比