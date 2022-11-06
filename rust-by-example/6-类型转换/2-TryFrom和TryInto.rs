// Author: yqq
// Date: 2022-11-06 10:05:55
// Description: TryFrom 和 TryInto



/*
pub trait TryFrom<T> {
    type Error;

    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T> {
    type Error;

    fn try_into(self) -> Result<T, Self::Error>;
}

*/

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct OddNum(i32);

impl TryFrom<i32> for OddNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if 1 == (value & 1) {
            Ok(OddNum(value))
        } else {
            Err(())
        }
    }
}


fn main() {

    for value in  vec![99, 20].iter() {
        match OddNum::try_from(*value) {
            Ok(v) => println!("{:?} try_from ok", v),
            Err(_) => println!(" {} try_from error", *value),
        }
    }


    for value in  vec![99, 20].iter() {
        let r: Result<OddNum, ()> = (*value).try_into();
        match r {
            Ok(v) => println!("{:?} try_into ok", v),
            Err(_) => println!("{} try_into error", *value),
        }
    }

}