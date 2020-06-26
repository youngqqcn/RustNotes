

fn main() {
    let s1 = String::from("");
    let s2 = s1; //此处将 s1 的所有权 移交给了 s2,  s1不再有效
    // println!("s1: {}", s1); //编译报错: value borrowed here after move
    println!("s2: {}", s2);
}