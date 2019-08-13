




/*
但是不能为外部类型实现外部 trait。例如，不能在多媒体聚合库
 crate 中为 Vec<T> 实现 Display trait。这是因为 Display 
 和 Vec<T> 都定义于标准库中，它们并不位于多媒体聚合库的 crate 
 本地作用域中。这个限制是被称为 相干性（coherence） 的程序属性
 的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不
 存在父类型。这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。
 没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，
 而 Rust 将无从得知应该使用哪一个实现。
*/

/*
pub trait  Summary{
    fn summarize(&self) -> String;
}



pub struct NewsArtice{
    pub headline : String,
    pub localtion : String,
    pub author : String,
    pub content : String,
}


impl Summary for NewsArtice{
    fn summarize(&self)->String{
        format!("{}, by{}({})", self.headline, self.author, self.localtion)
    }
}

pub struct Tweet{
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{} : {}", self.username, self.content)
    }
}



fn main() {

    let article =  NewsArtice{headline: String::from( "test"),
         localtion: String::from( "shenzhen"), 
         author: String::from("yqq"), 
         content:String::from("hello")};

    let tweet = Tweet{
        username: String::from( "yqq"), 
        content: String::from("happy"), 
        reply:false, retweet:true};


    println!("{}", article.summarize());
    println!("{}", tweet.summarize());




    // println!("Hello, world!");
}

*/




fn largest<T : PartialOrd + Copy> (list : &[T])->T{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }

    largest
}




fn main() {
    let lst_numbers = vec![15, 2, 53, -1, 3, -5, 22];
    let result =  largest(&lst_numbers);
    println!("{}",result );
    assert_eq!(result,  53);


    let lst_chars = vec!['3' , '1', 'z', 'A', ','];
    let result_char = largest(&lst_chars);
    println!("{}", result_char );
    assert_eq!(result_char, 'z');

}