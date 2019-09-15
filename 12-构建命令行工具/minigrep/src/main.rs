

/**
*author: yqq
*date: 2019/9/2 16:05
*descriptions:
*/

use std::env;
use std::fs;
use  std::process;
use std::error::Error;


use minigrep;
use minigrep::Config;




fn main() {


    /*
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);


    let query = &args[1];
    let filename = &args[2];


    println!("search for {}", query);
    println!("in file {}", filename);


    let contents = fs::read_to_string(filename)
        .expect("something went wrong reading the file.");

    println!("{}", contents);
    */

    //let args : Vec<String> = env::args().collect();
    //let (query, filename) = parse_config(  &args );

    //let cfg = parse_config( &args );
    let cfg =  Config::new( env::args() ).unwrap_or_else(|err|{
        eprintln!("problem parsing arguments:{}", err);
        process::exit(1);
    }) ;



    println!("{}", cfg.query);
    println!("{}", cfg.filename);

    println!("----------------------");

    if let Err(e) =  minigrep::run(&cfg){
        eprintln!("application error:{}", e);
        process::exit(1);
    }

//    println!("{}", cfg.query);
//    println!("{}", cfg.filename);
}
//
//struct Config {
//    query : String,
//    filename : String,
//}

//
//impl Config{
//    fn new (args : &[String]) -> Result<Config, &'static str>{
//
//        //检查参数
//        if args.len() < 3{
//            panic!("not enough arguments.");
//        }
//
//        let query = args[1].clone();
//        let filename  = args[2].clone();
//
//
//        Ok(Config{query, filename})
//    }
//}

/*
fn parse_config(args : &[String]) -> Config{

    let query = args[1].clone();
    let filename = args[2].clone();
    //(query, filename)

    Config{query, filename}

}
*/

//
//fn run(config : &Config)  -> Result<(), Box<dyn Error>> {
//    let contents = fs::read_to_string(&config.filename)
//        .expect("something went wrong reading the file.");
//
//    println!("with exit: \n{}", contents);
//
//    Ok(())
//}
//



