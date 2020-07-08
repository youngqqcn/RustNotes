// mod 关键字声明了模块，
//Rust 会在与模块同名的文件中查找模块的代码。
mod front_of_house;  

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}