


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn foo() {
    let some_u8 = 0u8;
    match some_u8 {
        1 => println!("one"),
        2 => println!("two"),
        _ => (),
    }
}


fn main() {
    // let some_num = Some(666);
    // // println!(some_num.);

    // let x: i8  = 666;
    // let y: Option<i8> = Some(777);

    // let sum = x + y;


    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(Option::None);


    if let Some(5) = five{
        println!("equal five");
    }else if let Some(6) = six {
        println!("equal six");
    }

}