

fn add_one(x: i32) -> i32 {
    x + 1
}


// 函数指针
type fnt = fn(i32) -> i32;

fn add_twice(fun: fnt, arg: i32 ) -> i32 {
    fun(arg) + fun(arg)
}


fn main() {

    let a = add_twice(add_one, 5);
    println!("a = {}", a);


    // 使用闭包
    let c = add_twice(|x: i32| -> i32{
        x + 1
    }, 5);

    println!("c = {}", c);


}