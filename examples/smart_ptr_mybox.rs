
use std::ops::Deref;
use std::mem::drop;


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)  // type `MyBox<{integer}>` cannot be dereferenced
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox!");
    }
}


fn foo(name: &str) {
    println!("hello , {}", name);
}


fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // println!("y = {}", y); //  `MyBox<{integer}>` cannot be formatted with the default formatter
    println!("y = {}", *y);



    let m = MyBox::new(String::from("rustman"));
    foo(&m);
    foo( &(*m)[..] );  // Rust 没有解引用强制多态则必须编写的代码

    /*
    (*m) 将 MyBox<String> 解引用为 String。接着 & 和 [..] 获取了整个 String
    的字符串 slice 来匹配 hello 的签名。没有解引用强制多态所有这些符号混在
    一起将更难以读写和理解。
    解引用强制多态使得 Rust 自动的帮我们处理这些转换。
    */



    let c = MyBox::new(String::from("this is tmp var"));
    drop(c);
    println!("-------------end of main --------------");
}