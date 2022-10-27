
pub trait Mammal {
    fn eat(&self);
}

struct Human;
struct Dog;
struct God {
    pub lifes: Vec<Box<dyn Mammal>>,
}

impl God {

    // fn live_multi_v2(m: &[ Box<dyn Mammal> ]){
    pub fn live_multi_v2(&self){
        for item in self.lifes.iter() {
            item.eat();
        }
    }
}

impl Mammal for Human {
    fn eat(&self) {
        println!("human eat");
    }
}


impl Mammal for Dog {
    fn eat(&self) {
        println!("Dog eat");
    }
}

// fn live_multi<T>(m: &[T]) where T: Mammal { // error
// fn live_multi(m: &[Box<dyn Mammal>]) {  // &[std::boxed::Box<(dyn Mammal + 'static)>]
// fn live_multi(m: &[Box<dyn Mammal>]) {
fn live_multi(m: &Vec<Box<dyn Mammal>>) {
    for item in m.iter() {
        item.eat();
    }
}


fn main() {


    // let god = God{
    //     lifes:
    //     vec![ 
    //         Box::new(Human{}), 
    //         Box::new(Dog{}) ,
    //     ] 
    // };

    // god.live_multi_v2();

    //如果不加类型注解, 编译器会将v的类型推导为: std::vec::Vec<std::boxed::Box<Human>>
    // let v = vec![ 
    //     Box::new(Human), 
    //     Box::new(Dog) ,
    // ]; 


    let v: std::vec::Vec<std::boxed::Box<(dyn Mammal)>>  = vec![ 
        Box::new(Human), 
        Box::new(Dog) ,
    ];

    live_multi(&v); //ok
    // live_multi(&v[..]); //ok

}
