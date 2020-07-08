

fn some_func(x: i32) -> Option<i32> {
    if x < 0 {
        return None;
    }
    Some(x + 1)
}


fn main() {

    for i in -3..3 {
        //对函数返回值进行检查
        match some_func(i) {
            None => println!("minus"),
            Some(n) => println!("{}", n),
        };
    }

    let n = 8;
    match n {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!(">=3"),  // _ 匹配所有的值
    };

    let n = Some(3);
    if let Some(3) == n {
        println!("3");
    }

    if let Some(num) = some_func(55) {
        println!("some_func return: {}", num);
    }


}
