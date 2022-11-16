// Author: yqq
// Date: 2022-11-16 21:49:07
// Description: 指示符

/*
block
expr 用于表达式
ident 用于变量名或函数名
item
pat (模式 pattern)
path
stmt (语句 statement)
tt (标记树 token tree)
ty (类型 type)

*/

macro_rules! create_function {

    ($fn_name: ident) => {
        fn $fn_name() {
            println!("you called {:?}", stringify!($fn_name))
        }
    }
}


create_function!(foo);
create_function!(goooooooooooooooooood);


macro_rules! print_result {

    ($expr_name: expr) => {

        // `stringify!` 把表达式*原样*转换成一个字符串。
        println!("{:?} = {:?}", stringify!($expr_name), $expr_name);
    }
}

fn main() {
    foo();
    goooooooooooooooooood();


    print_result!(1 + 2);


    print_result!({
        let x = 11;

        x * x + 2 * x + 1
    });
}