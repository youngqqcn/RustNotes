use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::LinkedList;
use std::collections::VecDeque;

fn main() {


    let mut scores = HashMap::new();
    scores.insert(String::from("yqq"), 55);
    scores.insert(String::from("tom"), 98);
    scores.insert(String::from("hack"), 12);

    // let yqq = scores.get(&String::from("yqq"));
    // yqq.take()
    // println!(yqq.unwrap());

    for (k, v) in &scores {
        println!("{}: {}" , k, v);
    }

    scores.insert(String::from("airo"), 12);
    scores.insert(String::from("airo"), 99);  // key相同时, 新值覆盖旧值
    println!("{:?}", scores); // 打印出 scores



    // key 不存在时进行插入
    scores.entry(String::from("Good")).or_insert( 555 );
    println!("{:?}", scores); // 打印出 scores



    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {

        //第一次进行插入, 第二次开始返回value的mut引用
        let count = map.entry(word).or_insert(0); 
        *count += 1;

    }

    println!("{:?}", map);
}