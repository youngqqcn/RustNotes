

use std::collections::HashMap;


/*
fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Red"), 90);


 
 
    let teams = vec![String::from("blue"), String::from("red"), String::from("yellow")];
    let vctscores = vec![10, 20, 30];

    // let mapscores : HashMap<_, _>  = teams.iter().zip(vctscores.iter()); //error
    let mapscores   = teams.iter().zip(vctscores.iter());



    let score = scores.get("blue"); //return Some
    // println!("{}", score);


    //for (k, v) in scores{   //移动
    for (k, v) in &scores{
        println!("{} : {}", k, v);
    }

    //error
    // value used here after move
    // for (k, v) in scores{
        // println!("{} : {}", k, v);
    // }

    for (k, v) in &scores{
        println!("{} : {}", k, v);
    }





    println!("Hello, world!");
}
*/




fn main(){

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(20); // 已经存在 Blue, 则不改变

    println!("{:?}", scores);

    //根据旧值更新一个值
    println!("----------------------" );

    let text = "hello world wonderful happy ";

    let mut maptmp = HashMap::new();

    for word in text.split_ascii_whitespace(){
        let count = maptmp.entry(word).or_insert(0); //如果不存在则插入
        *count += 1;
    }
    println!("{:#?}", maptmp);


}
