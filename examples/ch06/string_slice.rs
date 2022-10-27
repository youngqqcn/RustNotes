


fn main() {


    // let hello = "Здравствуйте";
    // let hello = "我爱中国!"; //13个字节, UTF-8 中文占用 3 个字节, ! 是英文感叹号, 占用一个字节
    let hello = "我爱中国！"; //15个字节, UTF-8 中文占用 3 个字节, ！ 是中文感叹号, 占用3个字节

    // let s = &hello[0..4]; //thread 'main' panicked at 'byte index 4 is not a char boundary; it is inside '爱'
    let s = &hello[0..6];
    let len = hello.len();

    println!("{string}", string=s);
    println!("Len={length}", length=len); // 13



    // 如果你需要操作单独的 Unicode 标量值，最好的选择是使用 chars 方法。
    // chars 方法会将字符串分开并返回 char 类型的值
    for ch in "北京烤鸭!Beijing Duck!".chars() {
        println!("{}", ch);
    }


}