use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println!("Hello, world!");

    println!("欢迎进入猜猜看!");
    println!("请输入你猜测的数字:");

    let rand_num = rand::thread_rng().gen_range(1, 100);

   
    loop {

        let mut guess_num = String::new(); // 关联函数(associate function),  C++中的静态方法
        io::stdin().read_line(&mut guess_num).expect("读取错误!");
    
        // 这种方式, 在输入无效数字情况下会导致程序崩溃退出
        // let number: u32 = guess_num.trim().parse().expect("please input a number ");

        // 使用 trim() 去掉 '\n'
        // 使用 parse 将将字符串转换为 u32
        // 使用 match 对  parse返回的  Result<F, F::Err> 进行匹配处理
        let number: u32 = match guess_num.trim().parse() {
            //处理无效输入
            Ok(num) => num,
            Err(err) => {
                println!("error:{}, please input right number", err);
                continue
            }
        };
    
        // 进行模式匹配
        match number.cmp(&rand_num) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("yes");
                break;
            } 
        }
        

        /*
        // 也可以使用使用 if ... else  
        if number < rand_num {
            println!("small");
        } else if number == rand_num {
            println!("yes");
            break;
        } else {
            println!("big");
        }
        */

    }
}
