#include <iostream>
#include <vector>
#include <string>
#include <utility>

int main() 
{
    // https://en.cppreference.com/w/cpp/utility/move

    // std::string str = "hello world";
    // std::vector<std::string> vct;

    // vct.push_back(std::move(str));
    // std::cout << vct[0] << std::endl;

    // std::cout <<"str: " << str << std::endl; // 空


    std::string strtmp = "c++";
    std::string strnew(std::move(strtmp));
    std::cout << "strtmp:" << strtmp << std::endl; //空
    std::cout << "strnew:" << strnew  << std::endl;

}