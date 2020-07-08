

pub fn mul_two(x: i32) -> i32 {
    x * 2
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    use super::*;

    #[test]
    fn test_mul_two() {
        assert_eq!(4 ,  mul_two(2));
    }
}
