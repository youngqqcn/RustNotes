



/*
fn main() {

    panic!("crash and close .......");

    println!("Hello, world!");
}

*/



/*
use std::fs::File;
use std::io::ErrorKind;

fn main(){

    let f = File::open("hello.txt");

    let f = match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file, but there was a problem:{:?}", e)
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error)
        },
    };
}
*/


/*
use std::fs::File;
use std::io::ErrorKind;


fn main(){

    let f = File::open("hello.txt").map_err(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("There was a problem create the file:{:?}", error);
            })
        }else{
            panic!("There was a problem opeing the file:{:?}", error);
        }

    });


}

*/


use std::io;
use std::io::Read;
use std::fs::File;




fn read_username_from_file() -> Result<String, io::Error>{
    let f = File::open("hello.txt");

    let mut f  = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string( &mut s ){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}

fn read_username_from_file_ex()->Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}



#[derive(Debug)]
pub struct Guess {
    value : i32,
}


impl Guess{
    pub fn new(value : i32) ->Guess{
        if (value < 1 || value > 100){
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess{
            value
        }
    }

    pub fn value(&self)->i32{
        self.value
    }
}

fn main(){
    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt").expect("Failed to  open hello.");

    // let ret = read_username_from_file();

    // let ret = read_username_from_file_ex();


    
    // let mut g = Guess{value:-1};
    // let mut g = Guess::new(-9);  //new 函数中测试失败, panic
    let mut g = Guess::new(9);

    println!("{:?}", g );
    println!("{:?}", g.value() );
    println!("{:?}", g.value );
}




