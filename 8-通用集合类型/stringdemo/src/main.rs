






/*
fn main() {

    let mut s = String::new();


    let data = "hellllllllllllll";

    let s = data.to_string();

    let s = "ini...........".to_string();


    let s = String::from("initsdffffffffffffff");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");



    println!("Hello, world!");
}
*/




/*

fn main(){

    let mut s = String::from("fooooooooooo");

    // s.push("hello");
    s.push('中');


    s.push_str("world");



    println!("{}", s);

}
*/



fn main(){

    let s1 = String::from("hello, ");
    let s2 = String::from("world");


    let s3 = s1 + &s2;  //ok,   s1:是移动    s2 是借用   //fn add(self, s: &str) -> String {
    // let s3 = &s1 + &s2;  // error
    // let s3 = s1 + s2;  // error


    println!("{}", s3);


    // println!("s1:{}", s1);  // moved
    println!("s2:{}", s2);



    let s = format!("{}-{}-{}", "hello", s2, s3);
    println!("{}", s );


}














