
// String 内部是有  Vec<u8> 进行封装

fn main() {

    let mut s1 = String::from("重庆火锅");
    s1.push('赞'); //Rust的字符指的是 Unicode字符, 而不单指是一个字节ascii
    s1.push_str("非常好吃!");

    s1 += "麻辣火锅!";  
    s1 = s1 + "天下一绝!";

    s1 = format!("{}中国雄起!, {}", s1, "棒棒儿!");
    
    println!("{}", s1);

    // println!("第一个字符: {}", &s1[0]);  // String 不支持索引操作

    let d = "麻辣小面!"; 
    // println!("{}", &d[0..1]); //ERROR, 1不是有效边界
    println!("{}", &d[0..3]); // 麻
    println!("length: {}", d.len()); //13 个字节: 一个汉字占3个字节 感叹号是英文占一个字节


    //这种方式遍历字符串是安全的
    for ch in d.chars() {
        println!("{}", ch);
    }


}