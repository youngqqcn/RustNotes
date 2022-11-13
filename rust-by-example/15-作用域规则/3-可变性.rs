// Author: yqq
// Date: 2022-11-13 09:45:14
// Description: 可变性




fn main() {

    let immutable = Box::new(3333u32);
    println!(" {}", immutable);

    // *immutable = 4; // cannoot assign


    let mut mutable_box = immutable;
    println!("{}", mutable_box);

    *mutable_box = 444;
    println!("{}", mutable_box);

}