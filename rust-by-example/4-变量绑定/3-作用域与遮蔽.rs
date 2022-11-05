// Author: yqq
// Date: 2022-11-05 22:51:51
// Description: 作用域



fn main() {


    let a = 0;

    {
        let a  = 1;
        println!("a = {}", a); // 作用域外部的a被内部的a 遮蔽

        let b = 2;
        println!("b = {}", b);
    }
    // println!("b out = {}", b); // error 出了作用域

    println!("a = {}", a);

    // 同样遮蔽前面的a
    let a = "hhh";
    println!("a = {}", a);
    
    let a = 9;
    println!("a = {}", a);

}