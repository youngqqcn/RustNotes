

## 第16章  多线程

并发编程(Concurrent programming) , 代表程序不同部分相互独立执行
并行编程(Parrallel programming), 代表程序不同部分同时执行

本章内容:

- 如何创建线程来同时运行多段代码
- 消息传递(Message passing) 并发, 其中通道(channel) 被用来在线程间传递消息.
- 共享状态(Shared state)并发, 其中多个线程可以访问同一片数据
- Sync 和 Send  trait, 将Rust的并发保证扩展到用户定义的以及标准库提供的类型中.




多线程常见问题:
- 竞争状态（Race conditions），多个线程以不一致的顺序访问数据或资源
- 死锁（Deadlocks），两个线程相互等待对方停止使用其所拥有的资源，这会阻止它们继续运行
- 只会发生在特定情况且难以稳定重现和修复的 bug



线程操作:
- `thread::spawn()`: 创建线程, 返回 `JoinHanle<T>`

- `JoinHandle.join().unwrap()`:  用于阻塞等待子线程结束


```rust
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

```

### 线程与 `move`闭包

```rust

use std::thread;

fn main() {

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { //使用 move 关键字强制获取它使用的值的所有权,  移动了所有权
        println!("Here's a vector: {:?}", v);
    });

    // 违反了借用规则,编译报错! 在编译时期避免了潜在bug
    // drop(v); //value used here after move

    handle.join().unwrap();

}
```

对比 C++11中的线程的问题:

```cpp
#include <iostream>
#include <thread>
#include <memory>
#include <string.h>
#include <chrono>

int main() 
{
    // char *p = new char[100]{0};
    char *p = new char[100]{0};
    auto thrd = std::thread([&](){
        std::this_thread::sleep_for( std::chrono::seconds(3));
        strcpy(p, "hello world!");
    });

    delete[] p; //直接释放!!
    p = nullptr;
    
    if(thrd.joinable())
    {
        std::cout << "waiting for thread finished." << std::endl;
        thrd.join();
    }
    std::cout << "start clean " << std::endl;

    // delete[] p;
    // p = nullptr;
    return 0;
}
```


### 线程间消息传递


Channel(通道), 借鉴了golang中的通道(`chan`)设计思想

Go语言的口号: “不要通过共享内存来通讯；而是通过通讯来共享内存。”（“Do not communicate by sharing memory; instead, share memory by communicating.”）

通道有两个部分组成: 发送者(transmitter) 和 接收者(receiver).  两者中任意一方先退出则通道被关闭.

mpsc: 多个生产者,单个消费者 (multiple producer, single consumer)

`std::sync::mpsc::channel` 是基于队列, 即先进先出. 所以接收方收到的顺序和发送顺序是一致.


#### 异步通道 `mpsc::channel`

```rust
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
```


#### 同步通道 `sync_channel`

```rust
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
```


## Mutex 与  Arc

Mutex(互斥器)使用, 任意时刻只允许一个线程访问某些数据. 

Rc: 引用计数智能指针只能用于单线程中, 不能再多线程间共享.

Arc: 原子引用计数智能指针可以用于多线程中, 可以安全在线程间共享

```rust

use std::sync::Mutex;

fn main() {

    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 { //创建10个线程
        let handle = std::thread::spawn(move || {
            //编译错误: value moved into closure here, in previous iteration of loop
            let mut num = counter.lock().unwrap();  //加锁
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

```


使用 多线程和多所有权

在下面的例子中, 

Rc<T> 并不能安全的在线程间共享。当 Rc<T> 管理引用计数时，它必须在每一个 clone 调用时增加计数，并在每一个克隆被丢弃时减少计数。Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断。在计数出错时可能会导致诡异的 bug，比如可能会造成内存泄漏，或在使用结束之前就丢弃一个值。我们所需要的是一个完全类似 Rc<T>，又以一种线程安全的方式改变引用计数的类型。


```rust

use std::sync::{
    Mutex, Arc
};

use std::rc::Rc;

/*
fn foo1() {
    let counter_mtx = Mutex::new(0);

    let mut threads = Vec::new();;

    for i in 0..10 {
        let hd = std::thread::spawn( move || {
            let mut num = counter_mtx.lock().expect("lock error");
            *num += 1;
        });

        threads.push(hd);
    }

    for thd in threads {
        thd.join().expect("join error");
    }

    println!("{:?}",  *counter_mtx.lock().expect("lock error"));
}
*/

/*
// `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
fn foo2() {
    let counter_rc = Rc::new( Mutex::new(0) );

    let mut threads = Vec::new();;

    for i in 0..10 {
        let cnt = counter_rc.clone();
        let hd = std::thread::spawn( move || {
            let mut num = cnt.lock().expect("lock error");
            *num += 1;
        });

        threads.push(hd);
    }

    for thd in threads {
        thd.join().expect("join error");
    }

    println!("{:?}",  *counter_rc.lock().expect("lock error"));
}
*/

fn foo3() {
    let counter_arc = Arc::new( Mutex::new(0) );

    let mut threads = Vec::new();

    for _ in 0..10 {
        let cnt = counter_arc.clone();
        let hd = std::thread::spawn( move || {
            let mut num = cnt.lock().expect("lock error");

            // pub fn lock(&self) -> LockResult<MutexGuard<T>> 
            // lock() 返回的是 MutexGuard ,离开作用域会自动 unlock
            // 需要解引用才能获取到 T
            *num += 1;
        });

        threads.push(hd);
    }

    for thd in threads {
        thd.join().expect("join error");
    }

    println!("{:?}",  *counter_arc.lock().expect("lock error"));
}

fn main() {

    // foo1();
    // foo2();
    foo3();
   
}

```


#### 死锁

C++ 死锁例子: 

```cpp
//C++ 死锁例子

#include <iostream>
#include <string>
#include <thread>
#include <mutex>
#include <chrono>

int main(int argc, char const *argv[])
{
    std::mutex mtx_a;
    std::mutex mtx_b;

    auto thd1 = std::thread([&]() {
        for(uint64_t i = 0; ;i++)
        {
            // 两个线程获取锁的顺序不同, 导致死锁
            mtx_b.lock();
            mtx_a.lock();
            std::cout << "thread A: "<< i << std::endl;
            mtx_a.unlock();
            mtx_b.unlock();
        }
    });

    auto thd2 = std::thread([&]() {
        for(uint64_t i = 0; ; i++)
        {
            mtx_a.lock();
            mtx_b.lock();
            std::cout << "thread B: " << i << std::endl;
            mtx_b.unlock();
            mtx_a.unlock();
        }
    });

    thd1.join();
    thd2.join();

    return 0;
}

```

C++17中可以使用 `scope_lock`, `lock_guard`管理锁

```cpp
int main(int argc, char const *argv[])
{
    std::mutex mtx_a;
    std::mutex mtx_b;

    auto thd1 = std::thread([&]() {
        for(uint64_t i = 0; ;i++)
        {
            // 两个线程获取锁的顺序不同, 导致死锁
            std::scoped_lock scopelock(mtx_a, mtx_b);
            std::cout << "thread A: "<< i << std::endl;
        }
    });

    auto thd2 = std::thread([&]() {
        for(uint64_t i = 0; ; i++)
        {
            std::scoped_lock scopelock(mtx_a, mtx_b);
            //std::scoped_lock scopelock(mtx_b, mtx_a);
            std::cout << "thread B: " << i << std::endl;
        }
    });

    thd1.join();
    thd2.join();

    return 0;
}
```

C++17中的`std::scoped_lock`源码(GUN C++):

```cpp
/** @brief A scoped lock type for multiple lockable objects.
 *
 * A scoped_lock controls mutex ownership within a scope, releasing
 * ownership in the destructor.
 */
template<typename... _MutexTypes>
class scoped_lock
{
public:
    explicit scoped_lock(_MutexTypes&... __m) : _M_devices(std::tie(__m...))
    { std::lock(__m...); } 

    explicit scoped_lock(adopt_lock_t, _MutexTypes&... __m) noexcept
    : _M_devices(std::tie(__m...))
    { } // calling thread owns mutex

    ~scoped_lock()
    {
std::apply([](_MutexTypes&... __m) {
    char __i[] __attribute__((__unused__)) = { (__m.unlock(), 0)... };
}, _M_devices);
    }

    scoped_lock(const scoped_lock&) = delete;
    scoped_lock& operator=(const scoped_lock&) = delete;

private:
    tuple<_MutexTypes&...> _M_devices;
};

/** @brief Generic lock.
 *  @param __l1 Meets Lockable requirements (try_lock() may throw).
 *  @param __l2 Meets Lockable requirements (try_lock() may throw).
 *  @param __l3 Meets Lockable requirements (try_lock() may throw).
 *  @throw An exception thrown by an argument's lock() or try_lock() member.
 *  @post All arguments are locked.
 *
 *  All arguments are locked via a sequence of calls to lock(), try_lock()
 *  and unlock().  If the call exits via an exception any locks that were
 *  obtained will be released.
 */
template<typename _L1, typename _L2, typename... _L3>
void
lock(_L1& __l1, _L2& __l2, _L3&... __l3)
{
    while (true)
    {
        using __try_locker = __try_lock_impl<0, sizeof...(_L3) != 0>; //从第0个开始
        unique_lock<_L1> __first(__l1);
        int __idx;
        auto __locks = std::tie(__l2, __l3...); //生成元组
        __try_locker::__do_try_lock(__locks, __idx); //不停的尝试
        if (__idx == -1)
        {
            __first.release();
            return;
        }
    }
}

template<typename _Lock>
inline unique_lock<_Lock>
__try_to_lock(_Lock& __l)
{ return unique_lock<_Lock>{__l, try_to_lock}; }

template<int _Idx, bool _Continue = true> //递归继续的偏特化
struct __try_lock_impl
{
    template<typename... _Lock>
    static void
    __do_try_lock(tuple<_Lock&...>& __locks, int& __idx)
    {
        __idx = _Idx; //初始 _Idx 为0, 每递归深入一次进行加 1
        //获取第_Idx个元素(锁),并尝试lock
        auto __lock = std::__try_to_lock(std::get<_Idx>(__locks)); 
        if (__lock.owns_lock()) //是否拥有锁的所用权?
        {
             // _Idx + 2来判断是否已经到了倒数第2个(之所以加2,是因为_Idx是从0开始的), 
             // 最后一个要递归终止, __cont为false, 
             // 即进入 __try_lock_impl<_Idx, false>::__do_try_lock
            constexpr bool __cont = _Idx + 2 < sizeof...(_Lock);

            using __try_locker = __try_lock_impl<_Idx + 1, __cont>;
            __try_locker::__do_try_lock(__locks, __idx);

            if (__idx == -1)
                __lock.release();
        }
    }
};

template<int _Idx>
struct __try_lock_impl<_Idx, false>   //递归终止的偏特化模板类
{
    template<typename... _Lock>
    static void
    __do_try_lock(tuple<_Lock&...>& __locks, int& __idx)
    {
        __idx = _Idx;
        auto __lock = std::__try_to_lock(std::get<_Idx>(__locks));
        if (__lock.owns_lock())
        {
            __idx = -1;
            __lock.release();
        }
    }
};

```



Rust死锁例子


```rust
use std::sync::{Mutex, Arc};

fn main() {

    let mtx_a = Arc::new( Mutex::new(0) );
    let mtx_b = Arc::new( Mutex::new(0) );

    let mtx_a_cp = mtx_a.clone();
    let mtx_b_cp = mtx_b.clone();
    let thd1 = std::thread::spawn(move || {
        for _ in 0..(1<<15) {

            //以不同的顺序获取锁, 导致死锁
            let mut guard_b = mtx_b_cp.lock().unwrap();
            let mut guard_a = mtx_a_cp.lock().unwrap();
            *guard_a += 1;
            *guard_b += 1;
            println!("threadA: a={:?}", guard_a);
            println!("threadA: b={:?}", guard_b);
        }
        
    });

    let thd2 = std::thread::spawn(move || {
        for _ in 0..(1<<15) {
            let mut guard_a = mtx_a.lock().unwrap();
            let mut guard_b = mtx_b.lock().unwrap();
            *guard_a += 1;
            *guard_b += 1;
            println!("threadB: a={:?}", guard_a);
            println!("threadB: b={:?}", guard_b);
        }
    });

    println!("waitting for thread finished");
    thd1.join().unwrap();
    thd2.join().unwrap();
}
```




#### 原子引用计数 `Arc<T>`
Arc: 原子引用计数（atomically reference counted）类型, Arc类型可以安全地在线程间共享
线程安全是以牺牲性能为代价.


C++读写锁

```cpp
#include <iostream>
#include <mutex>
#include <thread>
#include <shared_mutex>
#include <iomanip>
#include <vector>
#include <chrono>
// #include <stdlib.h>
#include <cstring>

int main()
{

    std::shared_mutex g_mutex;
    unsigned char g_shared_buf[4];

    auto writer = std::thread([&]() {
        std::cout << "writer线程开始" << std::endl;
        
        std::cout << "开始写入" << std::endl;
        for (int n = 0; n < 10; n++)
        {
            {
                // std::lock_guard  lock(g_mutex);
                std::unique_lock lock(g_mutex);  //写锁
                std::cout << "写入" << n+1 << std::endl;
                memset(g_shared_buf, n+1, sizeof(g_shared_buf));
            }
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
        }
    });

    // 自动推导
    // std::vector<decltype(writer)> vct_threads;
    // vct_threads.push_back(std::move(writer));

    std::vector<std::thread> vct_threads;
    // vct_threads.emplace_back(std::move(writer));
    vct_threads.push_back(std::move(writer));

    for (int i = 0; i < 10; i++)
    {
        vct_threads.emplace_back(std::thread([&, i]() {
            for (int n = 0; n < 10; n++)
            {
                {
                    std::shared_lock lock(g_mutex);  //读锁
                    std::cout << "thread" << i << ":";
                    for (int n = 0; n < sizeof(g_shared_buf); n++)
                    {
                        std::cout << (int)g_shared_buf[n];
                    }
                    std::cout << std::endl;
                }
                std::this_thread::sleep_for(std::chrono::milliseconds(10));
            }
        }));
    }

    // writer.join();

    for (auto &thd : vct_threads)
    {
        thd.join();
    }

    return 0;
}
```


Rust 读写锁

```rust
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
```



#### `Send trait` 和 `Sync trait`

- `Send trait`: 允许在线程间转移所有权, 几乎所有的Rust类型都是`Send`的, 也有例外, 如 裸指针(raw pointer), `Rc`,`Rc`只能用于单线程中
- `Sync trait`: 允许多线程访问, 即其值的引用可以安全地传递到多个线程中,  
    - 对于任意类型`T`, 如果`T`是`Sync`的, 那么`&T`也是`Sync`的. 
    - 完全由`Sync`类型组成的复合类型也是`Sync`的


#### 总结

RefCell<T> / Rc<T>  与  Mutex<T> / Arc<T> 的相似性:
RefCell<T> 可以改变 Rc<T> 中的内容那样，同样的可以使用 Mutex<T> 来改变 Arc<T> 中的内容。


Rust 提供了用于消息传递的通道，和像 Mutex<T> 和 Arc<T> 这样可以安全的用于并发上下文的智能指针。
类型系统和借用检查器会确保这些场景中的代码，不会出现数据竞争和无效的引用。
一旦代码可以编译了，我们就可以坚信这些代码可以正确的运行于多线程环境，
而不会出现其他语言中经常出现的那些难以追踪的 bug。并发编程不再是什么可怕的概念：无所畏惧地并发吧！


> - 首页: [README.md](../README.md)
> - 上一章:  [第15章-智能指针](./第15章-智能指针.md)
> - 下一章:[第17章-Rust面向对象编程特性](./第17章-Rust面向对象编程特性.md)