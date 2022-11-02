// Author: yqq
// Date: 2022-11-02 22:32:42
// Description: Debug trait


// 使用了 #[derive(Debug)] 自动推导实现
#[derive(Debug)]
struct MyStructure(f64);


// 对MyStructure再包一层， 依然可以自动推导
#[derive(Debug)]
struct Wrapper(MyStructure);



#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    gender: &'a str,
    age: u8,
}






fn main() {
    println!("{:?}", MyStructure(888.234234) );
    println!("{:#?}", MyStructure(888.234234) ); // 美化


    println!("{:?}", Wrapper(MyStructure(0.3424)));


    let stu = Student{name:"yqq", gender:"male", age:1};
    println!("{:?}", stu);


    // 高级玩法
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");
}