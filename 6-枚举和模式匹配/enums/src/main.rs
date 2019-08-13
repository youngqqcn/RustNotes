

/*

#[derive(Debug)]
enum IpVersion{
    V4,
    V6,
}


struct IpAddr{
    version : IpVersion,
    addr : String,
}




fn main() {

    let four = IpVersion::V4;
    let six = IpVersion::V6;

    println!("{:?}",  four);
    println!("{:?}",  six);


    //使用额外的结构体保存
    let home = IpAddr{
        version : IpVersion::V4,
        addr : String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        version : IpVersion::V6,
        addr : String::from("::1"),
    };

    println!("Hello, world!");
}
*/


/*
#[derive(Debug)]
enum IpAddr{
    V4(String),
    V6(String),
}


fn main(){

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1") );

    println!("{:?}", home);
    println!("{:?}", loopback);
}
*/


#[derive(Debug)]
enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}


impl IpAddr{

    fn show(&self){
        println!("{:?}", self);
    }
}


fn main(){

    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1") );

    println!("{:?}", home);
    println!("{:?}", loopback);

    home.show();
    loopback.show();


    let op : Option<i32> = Option::Some(34);
    //println!("{}", op );


}












