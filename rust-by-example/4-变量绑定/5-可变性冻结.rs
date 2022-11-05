// Author: yqq
// Date: 2022-11-05 23:01:55
// Description:  可变性冻结



fn main() {

    let mut a = 10;

    {
        // 可变性被冻结
        // let a = a;
        // a = 9999; // error

        // ok
        let mut a = a;
        a = 9999;
        println!("a = {}", a);
    }
    a = 666;
    println!("a = {}", a);
}