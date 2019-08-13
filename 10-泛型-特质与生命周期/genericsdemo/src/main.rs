


/*
fn larget_i32(list : &[i32])-> i32{

    let mut largest = list[0];

    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }

    largest
}


fn largest_char (list : &[char])->char{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }

    largest
}





fn main() {
    let lst_numbers = vec![15, 2, 53, -1, 3, -5, 22];
    let result =  larget_i32(&lst_numbers);
    println!("{}",result );
    assert_eq!(result,  53);


    let lst_chars = vec!['3' , '1', 'z', 'A', ','];
    let result_char = largest_char(&lst_chars);
    println!("{}", result_char );
    assert_eq!(result_char, 'z');


}
*/



/*

fn largest<T> (list : &[T])->T{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }

    largest
}




fn main() {
    let lst_numbers = vec![15, 2, 53, -1, 3, -5, 22];
    let result =  larget_i32(&lst_numbers);
    println!("{}",result );
    assert_eq!(result,  53);


    let lst_chars = vec!['3' , '1', 'z', 'A', ','];
    let result_char = largest_char(&lst_chars);
    println!("{}", result_char );
    assert_eq!(result_char, 'z');

}
*/




/*

//结构体中的泛型
struct Point<T>{
    x : T,
    y : T,
}

struct PointEx<T, U>{
    x : T,
    y : U,
}

enum PointPlus<T>{
    Some(T),
    None,
}


fn main(){
    let integer = Point{x:5,  y:10};
    let float = Point{x:1.0 , y: 4.9};
    let int_and_float = PointEx{x : 9, y:7.999};

}
*/


/*
//方法定义中的泛型



struct Point<T>{
    x : T,
    y : T,

}


impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main(){
    // let p = Point{x:5, y:10};
    // let p = Point();  //error
    // println!("{}", p.x() );


    let p = Point{x:4.3, y:19.1};
    println!("{}", p.distance_from_origin());
}

*/




struct Point<T, U> {
    x : T,
    y : U,
}


impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other : Point<V, W>) ->Point<T, W> {
        Point{
            x : self.x,
            y : other.y,
        }
    }
}


fn main(){
    let p1 = Point{x : 5, y : 11};
    let p2 = Point{x: "hello", y : 'w'};

    let p3 = p1.mixup(p2);

    println!("{}, {}", p3.x, p3.y );
}

























