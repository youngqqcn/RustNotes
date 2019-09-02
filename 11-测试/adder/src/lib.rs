#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn another(){
        panic!("this is fail test.");
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{length:8 , width: 7};
        let smaller = Rectangle{length:5, width: 1};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn find_bug_larger_can_hold_smaller() {
        let larger = Rectangle{length:8 , width: 7};
        let smaller = Rectangle{length:5, width: 1};

        assert!(larger.bug_can_hold(&smaller));

    }


    #[test]
    fn test_add_five() {
        assert_eq!(10, add_five(5));
        assert_ne!(10, add_five(6));
    }

    #[test]
    fn find_bug_test_add_five() {
        assert_eq!(5, add_five(9));
    }

    #[test]
    fn find_bug_test_add_five_custome_debug_info() {
        assert_eq!(6, add_five(9), "errrrrrrrrrrrrror:{} != {}", 6, add_five(9));
    }


    //---------------guess------------

    #[should_panic]
    #[test]
    fn greater_than_100() {
        Guess::new(999);
    }


    #[test]
    //期待指定的panic信息
    #[should_panic(expected = "Guess value must be between 1 and 100.")]
    fn greater_than_300(){
        Guess::new(301);
    }



    //使用Result进行测试
    #[test]
    fn use_result() -> Result<(), String>{
        // if 2 + 2 == 4{
        if 2 + 2 == 4{
            Ok(())
        }else{
            Err(String::from("2 + 2 != 4"))
        }
    }

}



#[derive(Debug)]
pub struct Rectangle{
    length : u32,
    width : u32,
}

impl Rectangle{
    pub fn can_hold(&self, other : &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    pub fn bug_can_hold(&self, other : &Rectangle) -> bool{
        self.length < other.length && self.width > other.width
    }
}


pub fn add_five(a : i32) -> i32{
    a + 5
}



//-------------------------
pub struct Guess {
    value : i32,
}


impl Guess{
    pub fn new(value : i32) -> Guess{
        if value < 1 || value > 100{
            panic!("Guess value must be between 1 and 100, got {}", value);
            //panic!("Guess value must be between 1 and 100." );
        }

        Guess{
            value
        }
    }
}

















