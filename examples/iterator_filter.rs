

#[derive(Debug)]
struct Shoe {
    size: u8,
    style: String,
}


fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size:u8) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}



struct Counter {
    count: u32,
}


impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}


// 为 Counter类型实现 Iterator trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        }else {
            None
        }
    }
}



fn main() {

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 12, style: String::from("json") },
        Shoe { size: 13, style: String::from("java") },
        Shoe { size: 19, style: String::from("python") },
        Shoe { size: 19, style: String::from("rust") },
        Shoe { size: 12, style: String::from("js") },
        Shoe { size: 13, style: String::from("php") },
        Shoe { size: 14, style: String::from("cpp") },
    ];

    let shoe_in_my_size = shoes_in_my_size(shoes, 19);

    println!("{:?}", shoe_in_my_size);



    let  counter = Counter::new();

    for it in counter {
        // println!("{}", *it);  // 会自动解引用, 不需要加 *
        println!("{}", it);
    }



    // [1, 2, 3, 4, 5] 和 [2, 3, 4, 5]   zip(Counter::new().skip(1))
    // (1, 2), (2, 3), (3, 4), 
    // [2, 6, 12, 20]   // map(|(a, b)| a * b)
    // [ 6 , 12 ]
    // 6 + 12 ==> 18
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                            .map(|(a, b)| a * b)
                            .filter(|x| x % 3 == 0)
                            .sum();

    println!("sum={}", sum);
}