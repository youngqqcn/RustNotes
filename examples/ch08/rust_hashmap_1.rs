
use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 199);


    let names = vec!["Alice", "Bob", "Coris"];
    let ages = vec![11, 10, 14];



    let cls: HashMap<_, _> = names.iter().zip(ages.iter()).collect();
    //使用 zip 方法来创建一个元组的 vector，其中 “Blue” 与 10 是一对
    println!("{:?}", cls); //{"Alice": 11, "Coris": 14, "Bob": 10}



    // 所有权
    let name = String::from("name");
    let age = String::from("age");
    let mut tmp = HashMap::new();

    // pub fn insert(&mut self, k: K, v: V) -> Option<V>
    // 从 insert 方法的函数签名来看, k, v 是move
    tmp.insert(name, age); //moved

    println!("{:?}", tmp);
    // println!("name={}, age={}", name, age);

    let s = String::from("name");
    let kv = tmp.get_key_value( &s );
    match kv {
        Some((k, v)) => {
            println!("key:{}, value:{}", k, v);
        },
        None => println!("not found"),
    }

    /*
    for (k,v) in tmp {  // 这种方式是移动! for循环之后tmp就无效了
        println!("k:{:?}, v:{:?}", k, v);
    }
    */

    for (k,v) in &tmp { //这种方式才是借用
        println!("k:{:?}, v:{:?}", k, v);
    }

    tmp.insert("name".to_string(), "messi".to_string());  //如果键已经存在, 则新值替换就值

    // 只有键不存在时才插入
    tmp.entry("name".to_string()).or_insert("news".to_string());

    println!("{:?}", tmp);




    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // pub fn or_insert(self, default: V) -> &'a mut V
        // 根据 or_insert 的函数签名
        // or_insert会返回一个值得可变引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}