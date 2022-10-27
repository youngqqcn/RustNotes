use hello_macro::HelloMacro;

// 构造 crate 和其中宏的惯例如下：对于一个 foo 的包来说，
// 一个自定义的派生过程宏的包被称为 foo_derive 。
//在 hello_macro 项目中新建名为 hello_macro_derive 的包。
use hello_macro_derive::HelloMacro;


#[derive(HelloMacro)]
struct Demo;



fn main() {
    // println!("Hello, world!");
    Demo::hello_macro();
}
