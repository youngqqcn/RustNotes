/*
理解Rust中的所有权
*/

/*
fn main() {
    // println!("Hello, world!");


    let mut str_tmp = String::from(" hello world ");
    println!("{}", str_tmp);

    str_tmp.push_str("fdffsff");
    str_tmp += "32423";
    println!("{}", str_tmp);

}
*/



/*
//1.移动
fn main(){
    let s1 = String::from("hell");
    let s2 = s1;

    println!(" {} ", s2); //ok
    //println!("  {} ", s1);  // error[E0382]: borrow of moved value: `s1`
}
*/


/*
// 2. 克隆
fn main(){

    let s1 = String::from("hwll");
    let s2 = s1.clone();

    println!("s1:{}, s2:{}", s1, s2);
}
*/


/*
//3.只在栈上的数据: 拷贝
fn main(){

    //整型这样的在编译时已知大小的类型被整个存储在栈上
    //所以拷贝其实际的值是快速的
    let x = 5;
    let y = x;
    let y = x.clone(); //等效

    println!("x = {}, y = {}", x, y);
}

*/



/*
所有整数类型，比如 u32。
布尔类型，bool，它的值是 true 和 false。
所有浮点数类型，比如 f64。
字符类型，char。
元组，当且仅当其包含的类型也都是 Copy 的时候。
    比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是

*/


/*
fn main(){
    let s = String::from("good");
    
    move_ownership(s); //这里是移动

    //println!("main():  {}", s); //error[E0382]: borrow of moved value: `s`

    
    let x = 555;
    make_copy(x);  //这里是copy
    println!("main():  {}",  x );  //ok
}

fn move_ownership(s : String){
    println!("move_ownership():  {}", s);
}


fn make_copy(n : i32){
    println!("make_copy():  {}", n );
}
*/




/*
fn main(){

    let s1 = give_ownership();

    println!("{}", s1);


    let s2 = String::from("hell rust");
    println!("{}", s2);

    let s3 =  move_and_gives_back(s2);  //移动之后有返回
    println!("{}", s3);
    // println!("{}", s2);  //error 

}



fn give_ownership() -> String{
    
    let str_tmp = String::from("fsdfsdf");
    println!("{}", str_tmp);
    //return str_tmp;
    str_tmp
}


fn move_and_gives_back(s : String) -> String{
    //return s
    s
}
*/




/*

fn main(){

    let s1 = String::from("heldfds");
    // let (s2, len) = calc_length(s1);
    let (len, s2) = calc_length(s1);
    println!("s1:{}  length: {}", s2, len);
}


fn calc_length(s : String) -> (usize , String){
// fn calc_length(s : String) -> ( String , usize ){
    // (s, s.len()) //error   , s先被移动
    (s.len(), s)
}

*/

/*
//引用
fn main(){

    let mut s1 = String::from("hello reference");

    let len = calc_length(&s1);

    println!("The length of {} is {}",  s1, len);


    change(& mut s1);
    println!("s1 after change(): {}", s1);

}

//引用传递    称为   "借用"
fn calc_length(s : &String) -> usize{
    //error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
    //s.push_str("fd"); //不能修改 , 引用默认是不可变的

    s.len()
}


//可变引用
fn change(s : &mut String){
    s.push_str("$$$");
}
*/

/*
大部分语言中变量任何时候都是可变的。
这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，
它可由这三个行为造成：

两个或更多指针同时访问同一数据。
至少有一个指针被用来写入数据。
没有同步数据访问的机制。


数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；
Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！
*/




/*
fn main(){

    let mut s = String::from("hellofdsf");

    {
        let r1 = &mut s;
    }

    //ok   同时使用不可变引用
    //let r2 = &s;
    //let r3 = &s;

    //同时使用不可变引用和可变引用
    let r2 = &s;
    let r3 = &mut s;  //error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable




    //同时使用可变引用, 不可以
    // let r2 = &mut s;
    // let r3 = &mut s; //error[E0499]: cannot borrow `s` as mutable more than once at a time

    //println!("{}, {}, {}", r2, r3, r4);
    println!("{}, {}", r2, r3 );

}

*/




/*
fn main(){

    let r1 =  dangle();

    println!("{}", r1);

}

//野引用
//error[E0106]: missing lifetime specifier
fn dangle() -> &String{
    let s =String::from("你好啊");
    &s
}
*/


//类似 C/C++中的这种方式
/*
#include <stdio.h>
#include <stdlib.h>
char *GetStr()
{
    char *buf = malloc(100);
    strcpy(buf, "hello");
    
    free(buf);

    return buf;
}

int main()
{
    char *pszTmp = GetStr();
    printf("%s\n", pszTmp);

    return 0;
}
*/




/*

fn main(){

    let r1 =  dangle();

    println!("{}", r1);

}

//移动所有权
//和上面的例子不同的是, 这里移动了所有权
fn dangle() -> String{
    let s =String::from("你好啊");
    &s
}

*/

/*
//类似以下情况
#include <stdio.h>
#include <stdlib.h>
char *GetStr()
{
    char *buf = malloc(100);
    strcpy(buf, "hello");
    return buf;
}

int main()
{
    char *pszTmp = GetStr();
    printf("%s\n", pszTmp);
    free(pszTmp);
    return 0;
}
*/




//slice

/*
fn first_word(s : &String) -> &str{

    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}



fn main(){

    let mut s  = String::from("hello Rust");

    //println!("{}", s[0..5]);  //error[E0277]: the size for values of type `str` cannot be known at compilation time
    println!("{}", &s[0..5]);
    println!("{}", &s[..s.len()]);


    println!("{}", first_word(&s)); //ok

    let word = first_word( &s);
    

    //当拥有某值的不可变引用时，就不能再获取一个可变引用
    s.clear();  //error

    println!("{}", word); 
}
*/



/*
fn first_word(s: &String) -> usize {
     let bytes = s.as_bytes();

     for (i, &item) in bytes.iter().enumerate() {
         if item == b' ' {
             return i;
         }
     }

     s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}
*/










fn first_word(s : &str) -> &str{

    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}



fn main(){

    let mut s  = String::from("hello Rust");

    //println!("{}", s[0..5]);  //error[E0277]: the size for values of type `str` cannot be known at compilation time
    println!("{}", &s[0..5]);
    println!("{}", &s[..s.len()]);


    println!("{}", first_word(&s[..])); //ok

    let word = first_word( &s[..]);
    

    let str_lieteral = "goooooooooooooooood  mooooring";

    println!("{}", first_word( &str_lieteral[..]) );

    println!("{}", first_word(str_lieteral));

    //当拥有某值的不可变引用时，就不能再获取一个可变引用
    // s.clear();  //error

    println!("{}", word); 
}



















