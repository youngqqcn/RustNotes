
use std::fmt;

/*
只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该
 trait。一个绕开这个限制的方法是使用 newtype 模式（newtype pattern），它涉及到在一个元组结构体（第五章 “用没有命名字段的元组
结构体来创建不同的类型” 部分介绍了元组结构体）中创建一个新类型。
*/

// newtype 模式
// 没有运行时消耗
struct Wrapper(Vec<String>);


impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))  // 拼接 
    }
}

/*
// 如果想要在 Vec<T> 上实现 Display，而孤儿规则阻止我们直接这么做，
因为 Display trait 和 Vec<T> 都定义于我们的 crate 之外。
// error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
impl fmt::Display for Vec<String> {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "hello world")  // 拼接 
    } 
}
*/


fn main() {

    let w = Wrapper(vec![
            String::from("hello"),
            String::from("world"),
            String::from("newboy"),
            ]);

    println!("w = {}", w);

}