#include <iostream>
#include <string>
#include <thread>
#include <mutex>
#include <chrono>


// int main()
// {
//     std::cout << "hello world" << std::endl;
//     return 0;
// }


// #if 0

int main(int argc, char const *argv[])
{

    std::mutex mtx;
    std::unique_lock<std::mutex>  lck(mtx);

    auto thd = std::thread([&](){
        std::this_thread::sleep_for(std::chrono::seconds(5));
        if(lck.owns_lock())
        {
            std::cout << "owns" << std::endl;
        }
        else 
        {
            std::cout << "not own" << std::endl;
        }
        // lck.release();
        lck.unlock();
    });

    for(int i = 0; i < 1000; i++)
    {
        try{
            if(lck.try_lock()){
                std::cout << "try_lock successed" << std::endl;
                break;
            }
        }catch(...) {
            std::cout << "try_lock failed" << std::endl;
        }

        std::this_thread::sleep_for(std::chrono::seconds(1));
    }


    thd.join();
    std::cout << "结束" << std::endl;

   
    return 0;
}

// #endif