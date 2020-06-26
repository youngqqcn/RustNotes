#include <iostream>
#include <string>


std::string &foo()
{
    std::string strTmp = "this is test";
    return strTmp;
}

struct Foo
{
    Foo(std::string &str): strRef(str) {}
    std::string &strRef;
};

int main()
{
    std::string &strref = foo();
    // std::cout << strref << std::endl; //error

    std::string strTmp = "This is a apple";
    Foo foo  = Foo(strTmp);
    std::cout << foo.strRef << std::endl;

    return 0;
}