use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::error;



fn read_string_from_file_v1(path: &String) -> Result<String, io::Error> {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file, // 返回文件句柄
        Err(error) => return Err(error), //直接返回
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn read_string_from_file_v2(path: &String) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}


fn read_string_from_file_v3(path: &String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}




fn main() -> Result<(), Box<dyn error::Error>> {

    let path = String::from("tmp.txt");
    /*
    let file_content = match read_string_from_file(&s) {
        Ok(content) => content,
        Err(error) => {
            println!("read file error: {}", error);
            return ;
        }
    };
    */

    /*
    let file_content =  match read_string_from_file_v2(&path) {
        Ok(content) => content,
        Err(error) => {
            println!("read file error: {}", error);
            return ; 
        }
    };
    */

    let file_content  =  read_string_from_file_v3(&path)?;
    println!("file content: {}", file_content);
    Ok(())
}