
use std::mem;


fn main() {


    let vct : [i32; 5] = [1, 2, 3, 4, 5];
    println!("first element:{}", vct[0]);
    println!("second element:{}", vct[1]);

    println!("array size : {}", vct.len());

    //栈
    println!("array capcity:{} bytes",  mem::size_of_val(&vct));


    analyze_slice( &vct);

    analyze_slice( &vct[3..]);

    
    println!("{}", vct[100]);
}



fn analyze_slice(slice : &[i32]) {
    println!("first element of the slice:{}", slice[0]);
    println!("the slice'size:{}", slice.len());
}