use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/*

trait Add<RHS=Self> { //默认参数类型
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
*/


//实现  + 运算符重载
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}



struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters{
        Millimeters(self.0 + (other.0 * 1000))
    }
}



trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!(" pilot fly up");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!(" wizard fly up");
    }
}


impl   Human {
    fn fly(&self) {
        println!("human fly");
    }
}


fn main() {
    
    let a = Point { x: 1, y: 0 };
    let b = Point { x: 2, y: 3 };
    // Point { x: 3, y: 3 }
    let c = a + b;
    println!("{:?}", c);

    let m = Millimeters(34);
    let n = Meters(2);

    let x = m + n;

    println!("{}", x.0);


    let person  = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

}