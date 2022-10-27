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