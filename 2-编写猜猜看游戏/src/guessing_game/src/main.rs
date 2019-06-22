




/*
什么是RLS ??

*/





// fn main() {
//     println!("Hello, world!");
// }


// fn main() {
//     println!("hello");
//     println!("hello");
// }


//use std::io;  


/*

fn main(){
    println!("Guess the number!");
    println!("please input your guess");


    let guess = "234234";
    println!("you guessed : {}", guess);
    //guess = 3432; //error[E0308]: mismatched types


    //在Rust中变量默认不可变
    //guess = "3432"; //error[E0384]: cannot assign twice to immutable variable `guess`


    // let mut strGuess = "2343"; //warning: variable `strGuess` should have a snake case name
    let mut str_guess  = "234324";
    //str_guess = 2349943; //error 类型不匹配
    println!("some word: {}", str_guess);

    str_guess = "3432948"; 

    println!("some word 2: {}", str_guess);

    
}
*/


use std::io; //使用标准io,  
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");


    let n_rand= rand::thread_rng().gen_range(1, 999); //生成[1, 999)区间的随机数
    //println!("number is : {}", n_rand);

    loop{
        println!("please input your guess"); //  println! 是一个宏

        let mut guess = String::new();  //new()是/String类型的 关联函数（associated function）, 相当于C++中的静态函数
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // std::io::stdin().read_line(&mut guess).expect("Failed to read line");

        //指定n_guess变量的类型 是 i32
        // trim 用于 去掉  '\n'
        // parse 从字符串转为数字,  返回值是  Result类型
        //let n_guess : i32 = guess.trim().parse().expect("Please type a number!");



    
        //处理无效的输入, 不让程序崩溃
        let n_guess : i32 = match guess.trim().parse(){
            Ok(n)=>n,
            Err(_) => continue,
        };


        match n_guess.cmp(&n_rand){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!" );
                break;
            }
        }

        //println!("you guessed : {}", guess);  // {} 是 占位符
    }
}

//使用 cargo doc --open   可以查看帮助文档







