

fn main() {
    let a = [1, 2, 3, 4];
    println!("{:?}", &a[..]); // OK
    println!("{:?}", &a[2..]); // OK

    let sa = vec![1, 2, 3, 4];
    println!("{:?}", &sa[..]); // OK
    println!("{:?}", &sa[2..]); // OK


    //元组不支持切片
    // let tp = (1, 2, 3, 4);
    // println!("{:?}", &tp[..]); // ERROR 
    // println!("{:?}", &tp[2..]); // ERROR

}