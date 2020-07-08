


/*
fn main() {
    let s1 = String::from("this is test string");
    let len = get_length( &s1 );
    println!("{}", len);
}


fn get_length(s : &String) -> usize {
    s.len()
}

*/




/*
fn main() {

    let mut s = String::from("thi si s");
    change_str( &mut s );

    println!("{}", s);

}


fn change_str(s : &mut String) {
    s.push_str("append");
}

*/



/*
fn main() {


    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // println!("{} and {}", r1, r2); // 不可变引用 和 可变引用  的作用于重叠

}


*/



fn main() {
    let s = dangle();
}

/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

fn dangle() -> String {
    let s = String::from("hello");
    s
}




