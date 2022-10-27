use rand::{thread_rng, Rng};

use std::cmp::Ordering;
use std::io;

fn main() {
    println!("game start, input `exit` to exit the game");

    loop {
        // 1、生成随机数，
        // 我们可以到https://crates.io搜索随机数的crate,复制配置项添加到Cargo.toml中
        // 可以搜索
        let mut rng = thread_rng();
        let rnd: u64 = rng.gen_range(0..10);
        println!("{}", rnd);

        // 2、获取用户输入的数
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().eq_ignore_ascii_case("exit")  {
                    println!("game over");
                    std::process::exit(0);
                }
            }
            Err(error) => println!("error: {error}"),
        }

        // 2.1 将字符串解析为整数
        let n: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("input error: {err} please try again");
                continue;
            }
        };

        // 3、对比用户输入
        match n.cmp(&rnd) {
            Ordering::Less => println!("<"),
            Ordering::Equal => println!("="),
            Ordering::Greater => println!(">"),
        }

        // 4、输出游戏结果
    }
}
