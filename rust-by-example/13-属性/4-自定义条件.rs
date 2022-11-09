#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}

// 如果使用  rustc 4-自定义条件.rs
//      not found in this scope

// rustc --cfg some_condition 4-自定义条件.rs