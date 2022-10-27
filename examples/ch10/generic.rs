

struct Point<T> {
    x : T,
    y : T,
}


struct PointEx<T, U> {
    x : T, 
    y : U,
}


impl<T> Point<T> {
    
    fn x(&self) -> &T {
        &self.x
    }

}

impl Point<f32> {

    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(3) + self.y.powi(2)).sqrt()
    }

}


impl<T, U> PointEx<T, U> {


    //此处使用的 move
    fn mixup<V, W>(self, other:PointEx<V, W>) -> PointEx<T, W> {
        PointEx {
            x : self.x,
            y : other.y,
        }
    }

}


fn main() {

    let integer = Point { x: 5, y: 10};

    let float = Point{ x: 1.0, y: 9.999};

    let string = Point{ x: "sfs", y: "dsfsd"};


    let px = PointEx {x: "sdfs", y: 9.9999}; //ok


    println!("{}", string.x());

    println!("distance: {}", float.distance_from_origin());



    let p1 = PointEx {x : 5,  y : 10.4};
    let p2 = PointEx {x:"hello", y:'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y={}", p3.x, p3.y);
    // println!("p2.x={}, p2.y={}", p2.x, p2.y);  // value borrowed here after move

    



}