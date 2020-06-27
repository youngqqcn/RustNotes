#include <iostream>
#include <map>
#include <string>


int main()
{
    std::map<std::string, int> mp;

    //返回类型: std::pair<std::map<std::string, int>::iterator, bool>
    auto ret = mp.insert(std::make_pair("name", 234));
    if(!ret.second)
    {
        std::cout << "插入失败" << std::endl;
    }
    else
    {
        std::cout << "插入成功" << std::endl;

        auto it = *ret.first;
        std::cout << it.first << " : " << it.second << std::endl;
    }

    ret = mp.insert(std::make_pair("name", 234));
    if(!ret.second)
    {
        std::cout << "插入失败" << std::endl;
    }
    else
    {
        auto it = *ret.first;
        std::cout << it.first << " : " << it.second << std::endl;
        std::cout << "插入成功" << std::endl;
    }
    



    



}