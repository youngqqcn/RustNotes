
fn foo(s: &String) {
    println!("foo() s = {}", s);
    // s.push_str("boy");  // ERROR 不可变引用,不可修改
}

fn oof(s: &mut String) {
    println!("oof() s = {}", s);
    s.push_str("boy"); //OK , 可变引用可以修改
}

fn main() {
    let mut s = String::from("hello");
    foo(&s); //借用

    println!("main() s = {}", s);

    oof(&mut s);
}