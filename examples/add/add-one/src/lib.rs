use rand;



pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    use super::*;  //使用父命名空间的

    #[test]
    fn test_add_one() {
        assert_eq!(3, add_one(2));
    }
}

