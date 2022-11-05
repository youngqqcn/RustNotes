// Author: yqq
// Date: 2022-11-05 22:56:12
// Description: 变量声明






fn main() {


    let a;

    {
        a = 999;
        println!("a = {}", a);
    }

    {
        // a = "hello"; // 因为a在前面已经绑定了，int， 所以这里不能再绑定其他类型
        println!("a = {}", a);
    }

    println!("a = {}", a);

    // 不能直接使用未定义的变量
    // let xxx;
    // println!("xxx is {}", xxx);

}