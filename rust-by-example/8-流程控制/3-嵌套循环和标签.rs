// Author: yqq
// Date: 2022-11-06 11:44:28
// Description:  嵌套循环 和 标签


//  有点类似汇编语言中的标签 ， 和 C/C++ Golang中的  goto ;

use std::io;



fn main() {
    println!("guess game start!");

    'outer: loop {
        let mut input = String::new();
        println!("please input a secret string:");
        io::stdin().read_line(&mut input).unwrap();
        let innn = input.as_str().trim();

        'inner: loop {
            println!("input your guess:");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).unwrap();

            let g = guess.as_str().trim();

            if g == "replay" {
                break 'inner;
            } else if g == "exit" {
                break 'outer;
            }

            if g == innn {
                println!("Congratulations!");
                break 'inner;
            } else {
                println!("Sorry please retry.")
            }
        }
    }
    println!("guess game over!")
}