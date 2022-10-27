use std::thread;
use std::time::Duration;


struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calc: T,
    value: Option<u32>,
}

// T 的 trait bound 指定了 T 是一个使用 Fn 的闭包
impl<T> Cacher<T> where T: Fn(u32) -> u32
{
    fn new(calc: T) -> Cacher<T> {
        Cacher{
            calc,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v, // 如果已经有值, 则直接获取, 
            None => { // 如果没值,则进行计算
                let v = (self.calc)(arg);
                self.value = Some(v); //缓存计算结果
                v //返回计算结果
            }
        }
    }

}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut slow_calc_result = Cacher::new(|num|{
        println!("calculating solwly....");
        thread::sleep(Duration::from_secs(3)); //sleep 3 secs
        num
    });

    if intensity < 25 {
        println!("do {} pushups", slow_calc_result.value(intensity));
        println!("next do {} situps", slow_calc_result.value(intensity));
    } else {
        if 3 == random_number {
            println!("take a break today!");
        } else {
            println!("Today, run for {} minutes",
                 slow_calc_result.value(intensity));
        }
    }
}






fn main() {
    // let add_one = |x: u32| {x + 1};
    // println!("{}", add_one(999));

    let rand_num = 99;

    generate_workout(32, rand_num);

}