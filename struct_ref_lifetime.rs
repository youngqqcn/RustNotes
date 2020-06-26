
use std::fmt;


struct ImportantExcerpt<'a> {
    // part : &'a str,
    part : &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn print_and_return(&self, inputstr : &str) -> &str {
        println!("output: {}", inputstr);
        self.part
    }
}

impl<'a> fmt::Display for ImportantExcerpt<'a> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.part)
    }
}


fn main() {

    let novel = String::from("this is a novel. I have a dream. ");
    let first_sentence = novel.split('.')
                               .next()
                               .expect("could not find a '.'");

    let item = ImportantExcerpt { part : first_sentence };


    println!("{}", item.level());
    println!("{}", item.print_and_return("happy"));

    println!("{}", item.part );


    let s : &'static str  = "I have a static lifetime";
    println!("{}", s);


    longest_with_an_display(novel.as_str(), &s, item);
}


fn longest_with_an_display<'a, T>(x : &'a str, y: &'a str, msg: T) 
        -> &'a str where T:fmt::Display{

    println!("msg: {}", msg);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}