// Author: yqq
// Date: 2022-11-21 22:33:44
// Description:





fn main() {

    let  collected_iterator: Vec<i32> = (0..10).collect();
    println!("{:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("{:?}", xs);
    xs.push(4);
    println!("{:?}", xs);


    // collected_iterator.push(0);
    println!("{}", xs.len());

    println!("{}", xs[1]);

    println!("{:?}", xs.pop());
    // println!("{}", xs[3]);


    for x in xs.iter() {
        println!(" {}", x);
    }


    for (i, x) in xs.iter().enumerate() {
        println!("{}, {}", i, x);
    }

    for x in xs.iter_mut() {
        *x  *= 10;
    }
    println!("{:?}", xs);
}