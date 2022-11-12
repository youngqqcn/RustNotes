// Author: yqq
// Date: 2022-11-12 23:04:45
// Description: 虚类型应用

use std::ops::Add;
use std::marker::PhantomData;


#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}


#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {

    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}



fn main() {

    let one_foot: Length<Inch> = Length(12.0 , PhantomData);

    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;


    // error , 两个不同的类型，  expected enum `Inch`, found enum `Mm`
    // let two_xx = one_foot + one_meter;

    println!("one foot + one_foot = {:?} in ", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
}