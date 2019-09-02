use std::env;
use std::fs;
use  std::process;
use std::error::Error;
use std::path::Prefix::Verbatim;


pub struct Config {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool,
}


impl Config{
    pub fn new (args : &[String]) -> Result<Config, &'static str>{

        //检查参数
        if args.len() < 3{
            //panic!("not enough arguments.");
            //Err("not enought arguments.")   //为什么不可用, 这样写?
            return Err("not enought arguments.");
        }

        let query = args[1].clone();
        let filename  = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();



        Ok(Config{query, filename, case_sensitive})
    }
}



pub fn run(config : &Config)  -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)
        .expect("something went wrong reading the file.");

//    println!("with exit: \n{}", contents);

    let results  =  if config.case_sensitive{
        search(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }


    Ok(())
}


fn search<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
//fn search(query : &str, contents : &str) -> Vec<&str> {
    let mut results = Vec::new();
//    let mut results :Vec<&str> = Vec::new();

    for line in contents.lines(){
        if line.contains(query ){
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a> (query : &str, contents : &'a str) -> Vec< &'a str>{

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains( &query ){
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn one_resulat(){
        let query = "duct";
        let content = "Rust:
safe, fast, productive.
Pick threee.";


        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );

    }


    #[test]
    fn case_insensitive(){

        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";


        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(  query, contents )
         );

    }



}







