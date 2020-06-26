
fn foo() -> String {
    let s = String::from("hello");
    s   //移出
}

fn back(s: String) -> String {
    println!("back() s = {}", s);
    s
}

fn oof(s: String) {   //获取所有权
    println!("oof() s = {}", s);
}

fn main() {

    let s1 = foo();
    println!("s1 = {}", s1); //OK

    let s2 = back(s1); // s1已经被moved, 

    oof(s2); //moved
    // 此处 s1已经无效

    // println!("s2 = {}", s2); //ERROR
}