//! # My Crate
//!
//! `my_crate` 是一个使得特定计算更方便的
//! 工具集合
/// 将给定的数字加一
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, cargodemo::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}