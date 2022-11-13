// Author: yqq
// Date: 2022-11-13 16:00:00
// Description: 结构体


// struct BB(&i32); // error， 必须注明 生命周期参数

#[derive(Debug)]
struct BB<'a>(&'a i32);



#[derive(Debug)]
struct NameBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Refff(&'a i32),
}


fn main() {

    let x = 18;
    let y = 15;

    let single = BB(&x);
    println!("x is borrow in {:?}", single);

    let dd = NameBorrowed { x: &x, y: &y };
    println!("====> {:?}", dd);

    let nn = Either::Num(444);
    println!("{:?}", nn);

    let rr = Either::Refff(&3333);
    println!("{:?}", rr);

}