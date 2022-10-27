

fn main() {

    let x = vec![1, 2, 3];

    let is_equal_to_x = |a| a == x; //borrow
    // let is_equal_to_x = move |a| a == x; // move

    if is_equal_to_x(vec![1, 2, 3]) {
        println!("yes");
    }
    // println!("x = {:?}", x);
}