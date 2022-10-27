

fn main() {

    let mut vct = vec![1, 2, 3, 5];
    for i in &vct {
        println!("{}", i);
    }


    for i in &mut vct {
        *i += 500;
    }

    for i in &vct {
        println!("{}", i);
    }


    

}