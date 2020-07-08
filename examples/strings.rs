

fn main() {
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");


    let mut hello = String::from("hello");

    let mut hi = hello + "good";   //String::add

    hi.push_str("this is a apple");

    println!("{}", hi);


    // hello.add(other: &str)

    /*

     #[inline]
    fn add(mut self, other: &str) -> String {  //转移了所有权
        self.push_str(other);
        self
    }

    */


    // 注意  + 运算符会移动所有权
    // 之所以能够在 add 调用中使用 &s2 是因为 &String 
    // 可以被 强转（coerced）成 &str。当add函数被调用时，Rust 使用了一个被称为 
    // 解引用强制多态（deref coercion）的技术，你可以将其理解为它把 &s2 
    // 变成了 &s2[..]。
    // let s3 = hello + &hi;  //hello.add( &hi);
    // println!("{}", s3);
    // println!("{}", hello); //  value used here after move

    // let s4 = hello + &hi; 
    // println!("{}", s3);


    let ss1 = String::from("ss1");
    let ss2 = String::from("ss2");
    let ss3 = String::from("ss3");
    let sss= format!("{}__{}__{}", ss1, ss2, ss3);
    println!("{}", sss);




}