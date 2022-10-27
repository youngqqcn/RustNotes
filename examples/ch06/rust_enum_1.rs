
enum Message {
    Quit,  //没有关联数据
    Move { x: i32, y: i32 },  //匿名结构体
    Write(String), //包含String
    ChangeColor(i32, i32, i32), //包含3个i32
}

// 可以为 枚举类型实现 方法
impl Message {
    fn call(&self) {
        println!("call()");
    }
}


fn main() {

    let m = Message::Write(String::from("good"));
    m.call();

}