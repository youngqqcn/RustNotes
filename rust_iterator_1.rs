

fn main() {

    let v1 = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|x| x * x).collect();

    println!("v2  = {:?}", v2);
}