use std::fs;
use std::fs::File;
use std::io::{self,  Read};


fn main() {


    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file ,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt"){
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };


    // unwrap_or_else : Returns the contained Ok value or computes it from a closure.
    // unwrap_or:  Returns the contained Ok value or a provided default.
    // or_else: Calls op if the result is Err, otherwise returns the Ok value of self.
    /*let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("create file error:{:?}", error);
            })
        } else {
            panic!("open file error:{:?}", error);
        }
    });
    */


    // expect 用来调用 panic! 的错误信息将会作为参数传递给 expect ，
    // 而不像unwrap 那样使用默认的 panic! 信息
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("failed to open hello.txt");



    // let ret = read_username_from_file().expect("read_username_from_file failed");   
    // println!("{}", ret.unwrap());
    // println!("{}", ret);


    // let s = read_username_from_file_ex();
    // let s = read_username_from_file_2();
    let s = read_username_from_file_3();
    // let s = read_username_from_file_ex();
    println!("{}", s.unwrap());

}




fn  read_username_from_file() ->Result<String, io::Error> {
    let fp = File::open("hello.txt");

    let mut f = match fp {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s =String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


/*
match 表达式与问号运算符所做的有一点不同：? 运算符所使用的错误值被传递给了 from 函数，
它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。
当 ? 运算符调用 from 函数时，收到的错误类型被转换为定义为当前函数返回的错误类型。
这在当一个函数返回一个错误类型来代表所有可能失败的方式时很有用，
即使其可能会因很多种原因失败。只要每一个错误类型都实现了 from 
函数来定义如将其转换为返回的错误类型，? 运算符会自动处理这些转换。
*/
fn read_username_from_file_ex() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // 使用 ? 运算符自动转换错误
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}





