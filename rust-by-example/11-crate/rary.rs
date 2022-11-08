// Author: yqq
// Date: 2022-11-08 22:35:41
// Description: 库


pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

// rustc --crate-type=lib rary.rs
