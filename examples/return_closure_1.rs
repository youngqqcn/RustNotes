

// Fn(i32) -> i32
// doesn't have a size known at compile-time
// the trait `std::marker::Sized` is not implemented for `(dyn std::ops::Fn(i32) -> i32 + 'static)`
//fn return_closure() -> Fn(i32) -> i32 {
//    |x| x + 1
//}



fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new( |x| x + 1 )
}

fn main() {

    let cls = return_closure();

    println!("{}", cls(666));

}