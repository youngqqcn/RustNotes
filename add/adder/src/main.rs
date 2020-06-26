use add_one;
use rand;
use mul_two;

fn main() {

    let num = 10;

    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));


    // 如果计算时出现溢出, main' panicked at 'attempt to multiply with overflow
    // println!("mul_two:{}", mul_two::mul_two(1<<31));

    println!("mul_two:{}", mul_two::mul_two(1<<31));
}
