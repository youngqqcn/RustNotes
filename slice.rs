







fn main() {

    let s = String::from("This is an apple!");

    let rs = &s[0..5];
    let rs2 = &s[6..];
    print!("{}\n", rs);
    print!("{}\n", rs2);
    

    let slc =  first_word( &s );
    println!("{}", slc);


    let s2 = String::from("hellworodl");
    let slc2 = first_word( &s2 );

    s2.clear(); 

    println!("{}", slc2);

}

fn first_word(s : &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}
