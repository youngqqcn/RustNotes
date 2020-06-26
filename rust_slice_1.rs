

//值接受 &String, 不接收 &str
fn first(s: &String) -> &str{
    &s[0..1]
}

//可以接受 &String , 也可以接受 &str
fn second(s: &str) -> &str {
    &s[0..1]
}

fn main() {
    let s = "hello world";
    let s1 = String::from("hello world");
    // println!("{}", first(&s)); //error
    println!("{}", first(&s1)); //OK

    println!("{}", second(&s)); //OK
    println!("{}", second(&s1)); //OK   &String 会自动转为 &str
    println!("{}", second(&s1[..])); //OK  即 &s1[..]

}