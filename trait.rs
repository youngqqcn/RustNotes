use std::fmt;
use std::cmp;
// use std::mem;

pub trait Summary {
    // fn summarize(&self) -> String;

    fn summarize(&self) -> String {
        format!("read more .....")

    }
}

pub struct NewArticle {
    pub headline: String, 
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}



// impl Trait 是语法糖
pub fn comman_notify(item: &impl Summary) {
    println!("comman : {}", item.summarize());
}


// trait bound 才是王道
pub fn generic_notify<T: Summary>(item : &T) {
    println!("generic_notify: {}", item.summarize());
}


impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.username, self.content)
    }
}



// 指定多个 trait bound
pub fn compose_notify<T: Summary + fmt::Display> (item : &T) {
    println!("item: {}", item);
    println!("compose_notify:{}", item.summarize());
}

// 使用 where简化 trait bound
pub fn compose_notify_ex<T> (item : &T) where T: fmt::Display + Summary {
    println!("item: {}", item);
    println!("compose_notify_ex:{}", item.summarize());
}


/*
//这里尝试返回 NewsArticle 或 Tweet。这不能编译，因为 impl Trait 工作方式的限制
// 第十七章的 “为使用不同类型的值而设计的 trait 对象” 部分会介绍如何编写这样一个函数。
pub fn comman_factory(is_tweet : bool) -> impl Summary {

    if is_tweet {
        Tweet {
            username: String::from("yqq"),
            content: String::from("this is a happy world."),
            reply: false,
            retweet: false,
        }
    } else {
        NewArticle {
            headline: String::from("I have a dream"),
            location: String::from("China"),
            author: String::from("yqq"),
            content: String::from("I have a dream.........."),
        }
    }

}
*/


// 获取最大值
fn largest<T> (lst : &[T]) -> T 
    where T: cmp::PartialOrd + Copy {

    let mut max = lst[0];
    for &item in lst.iter() {
        if item > max {
            max = item; //需要实现 Copy  trait
        }
    }
    max
} 




struct Pair<T> {
    x: T, 
    y: T, 
}


impl<T> Pair<T> {
    fn make_pair(x: T, y: T) -> Self {
        Self{
            x, 
            y,
        }
    }
}

// 仅为类型T实现了 Display 和PartialOrd trait 的 Pair<T>才会实现 cmp_display
impl<T> Pair<T> where T: fmt::Display + cmp::PartialOrd{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x={}", self.x);
        } else {
            println!("The largest member is y={}", self.y);
        }
    }
}





fn main() {

    let twt = Tweet {
        username: String::from("yqq"),
        content: String::from("this is a happy world."),
        reply: false,
        retweet: false,
    };


    let article = NewArticle {
        headline: String::from("I have a dream"),
        location: String::from("China"),
        author: String::from("yqq"),
        content: String::from("I have a dream.........."),
    };

    println!("1 new tweet:{}", twt.summarize());
    println!("1 new article:{}", article.summarize());


    comman_notify(&twt);

    generic_notify(&twt);
    generic_notify(&twt);


    compose_notify(&twt);
    compose_notify_ex(&twt);


    // let tmptwt = comman_factory(true);
    // println!("tmptwt:{}", tmptwt.summarize());


    let num_lst = vec![99, -1, 44, 2, 16, 29];
    let max = largest( &num_lst );
    println!("max = {}", max);

    let chr_lst = vec!['y', 'm', 'a', 'q'];
    let ch_max = largest( &chr_lst );
    println!("ch_lst max:{}", ch_max);


    let pair1 = Pair::make_pair(99, 555);
    // let pair2 = Pair::make_pair(44, 888);
    pair1.cmp_display(); // ok , 因为  i32 实现了 Display 和 PartialOrd


    // let pair2 = Pair::make_pair(article, article);
    // pair2.cmp_display();  // error ,  method not found in `Pair<NewArticle>`

}

