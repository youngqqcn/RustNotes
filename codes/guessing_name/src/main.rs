use std::io;
use rand::prelude::*;

use std::cmp::Ordering;

fn main() {

    // let mut guess_number = guess_str.parse::<i32>();

    let secret :i32 = rand::thread_rng().gen_range(0..100);
    loop {
        // let mut guess_str = String::from("");
        // match io::stdin().read_line(&mut guess_str) {
        //     Ok(_) => println!(""),
        //     Err(e) => println!("error {}", e)
        // }
        let mut guess_str = String::from("");
        io::stdin().read_line(&mut guess_str).expect("invalid number");

        // let mut guess_number : i32 = 0;
        // match guess_str.parse::<i32>() {
        //     Ok(n) => guess_number = n,     // NOT WORKING
        //     Err(_) => println!(""),
        // }


        // let guess_number = guess_str.trim().parse::<i32>().expect("hhh");

        let guess_number : i32 = match  guess_str.trim().parse::<i32>() {
            Ok(n) => n,
            Err(e) => {
                println!("invalid number {}", e);
                continue;
            }
        };


        // println!("secrete number is {}", secret);
        // println!("guess number is {}", guess_number);
        // if guess_number < secret {
        //     println!("low");
        // }else if guess_number == secret {
        //     println!("ok");
        //     break;
        // }else {
        //     println!("high");
        // }

        match guess_number.cmp(&secret) {
            Ordering::Less => println!("low"),
            Ordering::Greater => println!("high"),
            Ordering::Equal => {
                println!("ok");
                break;
            }
        }
    }


    println!("Hello, world!");
}
