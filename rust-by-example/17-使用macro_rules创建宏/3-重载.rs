// Author: yqq
// Date: 2022-11-16 22:03:46
// Description: 重载


// 宏可以重载，从而接受不同的参数组合。在这方面，
// macro_rules! 的作用类似于 匹配（match）代码块：



macro_rules! test {

    ($left: expr; and666 $right: expr) => (
        println!("{:?} and {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right
            )
    );

    ($left: expr; or777 $right: expr) => (
        println!("{:?} and {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left || $right
            )
    );

}

fn main() {

    test!(1 + 1 == 2; and666  2 * 4 == 8i32  );

    test!(true; or777 false);

}