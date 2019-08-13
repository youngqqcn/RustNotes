

/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

*/


/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    let result;// = longest(string1.as_str(), string2.as_str());
    {
        let string2 = String::from("xyz");

        //error[E0597]: `string2` does not live long enough
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
*/

/////////////////////////////////////

/*
fn longest<'a> (x : &'a str, y : &str) -> &'a str{
    x
}
*/

/*
fn longest<'a> (x : &str, y : &str) -> &'a str{
    // x //error[E0621]: explicit lifetime required in the type of `x`
    let result = String::from("dsffffffffffffskjlsdf");
    result.as_str()
}

fn main(){
    let str1 = String::from("goood");
    let str2 = String::from("sdfffffd");
    println!("{}" , longest(&str1, &str2) );
}
*/



/*
struct ImportExcerpt<'a>{
    part : &'a str,
}


fn main(){
    let novel = String::from("sdfksdfj.lsdflk...");
    let first_sentence = novel.split('.').next().expect("could not found a '.'");
    let  mut str_tmp = ImportExcerpt{part : first_sentence};
    println!("{}", str_tmp.part);
    // let second_sentence = novel.split('.').next().expect("could not found a '.'");
    // str_tmp = ImportExcerpt{part : second_sentence};
    // println!("{}", str_tmp.part);

}
*/


/*
这些规则适用于 fn 定义，以及 impl 块。

第一条规则是每一个是引用的参数都有它自己的生命周期参数。
换句话说就是，有一个引用参数的函数有一个生命周期参数：
fn foo<'a>(x: &'a i32)，有两个引用参数的函数有两个不同的生命周期参数，
fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。

第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
：fn foo<'a>(x: &'a i32) -> &'a i32。

第三条规则是如果方法有多个输入生命周期参数，
不过其中之一因为方法的缘故为 &self 或 &mut self，
那么 self 的生命周期被赋给所有输出生命周期参数。
这使得方法更容易读写，因为只需更少的符号。
*/



/*
//方法定义中的生命周期注解
struct ImportantExcerpt<'a>{
    part : &'a str,
}


impl<'a> ImportantExcerpt<'a>{
    fn level(&self)->i32{
        3
    }


    fn announce_and_return_part(&self, announce : &str) -> &str{
        println!("attention please: {}", announce);
        self.part
    }
}


fn main(){
    let tmpObj = ImportantExcerpt{part : "hello"};
    println!("{}", tmpObj.announce_and_return_part("gooooooooooood"));
}
*/

/*

//静态生命周期
//所有的字符串字面值都拥有 'static 生命周期

fn main(){
    let s : &'static str  = "this is a static lifetime.";
    println!("{}", s );
}
*/
use std::fmt::Display;

use std::fmt::Formatter;


fn longest_with_an_announcement<'a, T>(x : &'a str, y: &'a str, ann : T) 
    -> &'a str
    where T : Display  //支持实现了 Display trait的泛型
{
    println!("Announcement:{}", ann );
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

/*
struct Student <'a>{
    name : &'a str,
}

impl<'a> Display for Student<'a>{
    fn fmt(&self, f: &mut Formatter){
        println!("my display: {}",  self.name);
        // Result(Some)
    }
}
*/


fn main(){


    println!("{}", longest_with_an_announcement("tests", "sdfsdfsdfsd", 999));

    // let stu = Student{name : "yqq"};
    // println!("{}", stu.fmt(f: &mut Formatter));
    // println!("{}", longest_with_an_announcement("tests", "sdfsdfsdfsd", &stu));
}























