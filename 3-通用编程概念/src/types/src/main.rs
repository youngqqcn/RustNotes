
fn main() {


    let x = 2.0;


    //println!("%.2f", x); //error
    println!("{:.2}", x); //保留2位小数


    //数值运算
    let _sum = 5 + 999;


    let _dif = 9923.342 - 234.23;


    let _product = 4 * 234;


    let  _quotient = 234.234 / 234.2;


    let _remainder = 234 % 23;


    //布尔类型

    let _b_flag = true;

    let _b_flag : bool = false;



    //字符串类型
    //Rust中的char 代表一个 Unicode 标量, 包括以下内容: 
    //    中文
    //    日文
    //    韩文
    //    Emoji
    //    空白字符 ""
    //Unicode标量值包含:   U+0000 到  U+D7FF   U+E0000 到 U+10FFFF
    let c  = 'a';
    let z = 'Z';

    let heart_eyed_cat = '😻';

    let chinese_character = '中';

    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat);
    println!("{}", chinese_character);

    // 复合类型
    //tuple
    let tup_test : (i32, f64, u8)  = (123, 4.2, 1);

    let (x, y, z ) = tup_test;  // pattern matching  模式匹配
    println!("{}", y);

    let n_tmp = tup_test.0; //获取第0个元素
    let n_tmp = tup_test.1; //获取第1个元素
    let n_tmp = tup_test.2; //获取第2个元素

    //array

    //长度固定
    //let arr = [1, 2, 3, 4, 5];
    let arr : [i32; 5] = [1, 2, 3, 4, 5];
    // let arr : [i32; 6] = [1, 2, 3, 4, 5]; // error

    let n_tmp = arr[0];
    let n_tmp = arr[1];
    let n_tmp = arr[2];
    let n_tmp = arr[3];
    let n_tmp = arr[4];
    //let n_tmp = arr[10]; //error , 编译时错误
    println!("{}", n_tmp);


    let index = 10;
    println!("{}",  arr[index]); //运行时(runtime) 错误,   数组越界





}
