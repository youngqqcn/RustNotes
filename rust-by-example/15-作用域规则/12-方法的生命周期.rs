// Author: yqq
// Date: 2022-11-13 15:54:53
// Description: 方法的生命周期

// 方法一般是不需要标明生命周期的，
// 因为 self 的生命周期会赋给所有的输出生命周期参数，详见 TRPL。

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }

    fn print(&self) {
        println!("===> {}", self.0);
    }
}

/*

impl Owner {
    // 标注生命周期，就像独立的函数一样。
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

*/


fn main() {

    let mut owner = Owner(18);
    owner.add_one();
    owner.print();
}