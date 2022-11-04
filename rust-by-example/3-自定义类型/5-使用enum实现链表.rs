// Author: yqq
// Date: 2022-11-04 22:11:48
// Description: 使用enum实现链表

enum List {
    Cons( Box<List>, u32),
    Nil
}


use List::*;

impl List {


    fn new() -> List {
        // List::Nil
        Nil
    }

    fn push_back(self, v: u32) ->List {
        // List::Nil
        Cons( Box::new(self), v)
    }

    fn len(&self) -> u32 {

        // 牛逼！！！
        match *self {
            Cons(ref tail, _) => 1 + tail.len(),  // 递归下去
            Nil => 0 , // 递归结束
        }

    }

    fn stringify(&self) -> String {

        match *self {
            Cons( ref tail, v) => {
                let mut s = format!("{}", tail.stringify());
                if s != "" {
                    s += ",";
                }
                s = format!("{}{}", s, v);
                s
            }
            Nil => String::from(""), // 递归结束
        }

    }

}


fn main() {

    let mut l = List::new();
    l = l.push_back( 5);
    l = l.push_back( 3);
    l = l.push_back( 2);
    l = l.push_back( 1);

    println!("len: {}", l.len());

    println!("{}", l.stringify());
}