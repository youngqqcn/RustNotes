/**
 * C++ 死锁例子
 * 
 */

#include <iostream>
#include <string>
#include <thread>
#include <mutex>
#include <chrono>

/*
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
*/



int main(int argc, char const *argv[])
{
    std::mutex mtx_a;
    std::mutex mtx_b;

    auto thd1 = std::thread([&]() {
        for(uint64_t i = 0; i < 10000000 ;i++)
        {
            // 两个线程获取锁的顺序不同, 导致死锁
            std::scoped_lock scopelock(mtx_a, mtx_b);
            std::cout << "thread A: "<< i << std::endl;
        }
    });

    auto thd2 = std::thread([&]() {
        for(uint64_t i = 0; i < 10000000; i++)
        {
            // std::scoped_lock scopelock(mtx_a, mtx_b);
            std::scoped_lock scopelock(mtx_b, mtx_a);
            std::cout << "thread B: " << i << std::endl;
        }
    });

    thd1.join();
    thd2.join();

    return 0;
}
