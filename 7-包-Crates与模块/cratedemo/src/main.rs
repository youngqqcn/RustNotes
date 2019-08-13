

/*
mod sound {
    pub mod instrument{

        pub fn guitar(){
            println!("guitar.......");
        }


        //使用相对路径方式
        pub fn call_show(){
            super::super::show();  //   ../../show
        }
    }
}


fn show(){
    println!("show........" )
}




fn main() {
    // println!("Hello, world!");


    //使用绝对路径方式调用
    crate::sound::instrument::guitar();

    crate::sound::instrument::call_show();

}
*/



/*
mod plant {
    pub struct Vegetable{
        pub name  : String,
        id : i32,
    }

    impl Vegetable{
        pub fn new (name : &str) -> Vegetable {
            Vegetable {
                name : String::from(name),
                id : 1,
            }
        }
    }
}




fn main(){
    let mut v = plant::Vegetable::new("tomato");
    println!("{} are delicious", v.name);
    v.name = String::from("onion");
    println!("{} are delicious", v.name);

    // println!("id is {}", v.id); //error

}
*/



//枚举类型使用  pub
/*
mod menu{
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


fn main(){
    let order1 =  menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
*/



//使用use  和 绝对路径   

/*
mod sound {
    pub mod instrument {
        pub fn piano() {
            println!(" piano........");
        }
    }
}


use crate::sound::instrument;



fn main(){
    instrument::piano();
}

*/



/*

//使用use 和  相对路径
mod sound {
    pub mod instrument {
        pub fn piano() {
            println!(" piano........");
        }
    }
}

mod performance_group{
    // use crate::sound::instrument; //ok
    // use self::sound::instrument; //error
    use super::sound::instrument; //ok

    pub fn play_piano(){
        instrument::piano();
    }
}

fn main(){

    performance_group::play_piano();

}


*/



/*
// use 函数路径 (不推荐使用)
mod sound {
    pub mod instrument {
        pub fn piano() {
            println!(" piano........");
        }
    }
}

use crate::sound::instrument::piano;
fn main(){
    piano();
}

*/



/*
use std::fmt;
use std::io;


fn func1() -> fmt::Result{
    Ok(())
}

fn func2() -> io::Result<()>{
    Ok(())
}


fn main(){

    func1();
    func2();

}
*/


/*
use std::fmt::Result;
use std::io::Result;


fn func1() -> Result{
    Ok(())
}

fn func2() -> Result<()>{
    Ok(())
}


fn main(){

    func1();
    func2();

}
*/

/*
use std::fmt::Result;
use std::io::Result as IoResult;


fn func1() -> Result{
    Ok(())
}

fn func2() -> IoResult<()>{
    Ok(())
}


fn main(){

    func1();
    func2();

}
*/



/*
//重导出
//pub  和  use

mod sound {
    pub mod instrument {
        pub fn piano() {
            println!(" piano........");
        }
    }
}

mod performance_group{
    // use crate::sound::instrument; //error 
    pub use crate::sound::instrument; //ok

    pub fn play_piano(){
        instrument::piano();
    }
}

fn main(){

    performance_group::play_piano();

    performance_group::instrument::piano(); 

}

*/



// use std::cmp::Ordering;
// use std::io;

// use std::{cmp::Ordering, io};






// use std::io;
// use std::io::Write;

// use std::io::{self, Write};





//使用sound模块

mod sound;


fn main(){

    crate::sound::instrument::piano();


}

















