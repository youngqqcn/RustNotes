use std::cmp::PartialOrd;
// use std::c


fn largest<T>(v: &[T] ) -> T  where T: PartialOrd + Copy {
    let mut max = v[0];
    for &item in v.iter() {
        if item > max {
            max = item;
        }
    }

    max
}



fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}