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

    delete[] p;
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