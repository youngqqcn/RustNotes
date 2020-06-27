
enum  MyEnum{
    Int(i32),
    Float(f64),
    Txt(String),
}

fn main() {

    let v2: Vec<f64> = Vec::new();
    let mut v3 = vec![1, 3, 9]; //使用宏, 也是 Vec<iew> 类型

    for it in &mut v3 {
        *it += 100;
    }

    for it in v3 {
        println!("{}", it);
    }


    let vct = vec! [
        MyEnum::Float(1.234),
        MyEnum::Int(12),
        MyEnum::Txt(String::from("good")),
    ];

}