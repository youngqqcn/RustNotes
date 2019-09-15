



use std::thread;
use std::time::Duration;


fn main() {
//    println!("Hello, world!");


    /*
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
    */


    /*
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5.to_string());
    */



    let x = vec![1, 2, 3];

    let equal_to_x = move | z | z == x;  //将x的所有权移动到闭包中


    println!("{:?}", x);  //此处不能再用x

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

}




/*
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num : u32| -> u32{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
*/




/*

FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，
    environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义
    闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同
    变量的所有权的事实，所以它只能被调用一次。

FnMut 获取可变的借用值所以可以改变其环境

Fn 从其环境获取不可变的借用值

*/


struct Cacher<T> where T : Fn(u32) -> u32
{
    calc: T,
    value : Option<u32>,
}

impl<T> Cacher<T> where T : Fn(u32) -> u32
{
    fn new(calc : T) -> Cacher<T>{
        Cacher{
            calc,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_closure = Cacher::new(|num|{

        println!("calculating slowly............");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}
