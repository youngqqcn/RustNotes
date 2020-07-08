

## 第10章 泛型, trait, 生命周期


### 泛型

- 在函数定义中使用泛型
- 在结构体中使用泛型
- 在枚举中使用泛型
- 方法中使用

```rust
use std::cmp::{PartialOrd, PartialEq} ;
// use std::mem::


// 在函数定义中使用泛型
fn largest<T: PartialOrd>(v: &[T]) -> &T {
    let mut max = &v[0];
    for item in v.iter() {
        if item > &max {
            max  = item;
        }
    }
    max
}


//结构体中使用泛型
struct Point<X, Y> {
    x: X, 
    y: Y,
}

impl<X, Y> Point<X, Y> {
    // 方法中使用泛型
    fn mixup<XX, YY>(self, other: Point<XX, YY>) -> Point<X, YY> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}



fn main() {

    let v = vec![1, -1, 3, 9, 4];
    if let x =  largest(&v) {
        println!("x = {}", x);
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

```

泛型代码的性能: 
Rust在编译时对泛型代码进行*单态化*, 即为不同的类型生成相应的代码,  如C++中对泛型的第一次编译.



### trait


trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

注意：trait 类似于其他语言中的常被称为 接口（interfaces）的功能，虽然有一些不同。


实现 trait 时需要注意的一个限制是，只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。

但是不能为外部类型实现外部 trait。


这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。



因为向编译器提供了 trait bound 信息, 编译器就可以检查具体类型的行为, 相比动态语言在运行时出现错误,Rust将这些错误移到了编译时, 另外, 无需在运行时检查方法是否存在, 因为编译时已经检查过.



个人心得:
Rust语言部分包含了很多和编译器打交道的内容, 相比其他语言来说 , Rust要求程序员非常清楚自己想要什么和在做什么,并且把这些告诉编译器,让编译器协助检查,保证正确性.



使用 trait bound 实现 上面的  `largest` 函数, 和上面的版本差不多, 增加了 Copy 的trait bound , for 循环中使用 `&item`引用

```rust
use std::cmp::PartialOrd;
// use std::c


fn largest<T>(v: &[T] ) -> T  where T: PartialOrd + Copy {
    let mut max = v[0];
    for &item in v.iter() {
        if item > max {
            max = item;
        }
    }

    max
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

```


#### trait 对象

后文有trait的内容: https://kaisery.github.io/trpl-zh-cn/ch17-02-trait-objects.html


```rust
pub trait Mammal {
    fn eat(&self);
}

struct Human;
struct Dog;
struct God {
    pub lifes: Vec<Box<dyn Mammal>>,
}

impl God {

    // fn live_multi_v2(m: &[ Box<dyn Mammal> ]){
    pub fn live_multi_v2(&self){
        for item in self.lifes.iter() {
            item.eat();
        }
    }
}

impl Mammal for Human {
    fn eat(&self) {
        println!("human eat");
    }
}


impl Mammal for Dog {
    fn eat(&self) {
        println!("Dog eat");
    }
}

// fn live_multi<T>(m: &[T]) where T: Mammal { // error
// fn live_multi(m: &[Box<dyn Mammal>]) {  // &[std::boxed::Box<(dyn Mammal + 'static)>]
// fn live_multi(m: &[Box<dyn Mammal>]) {
fn live_multi(m: &Vec<Box<dyn Mammal>>) {
    for item in m.iter() {
        item.eat();
    }
}


fn main() {


    // let god = God{
    //     lifes:
    //     vec![ 
    //         Box::new(Human{}), 
    //         Box::new(Dog{}) ,
    //     ] 
    // };

    // god.live_multi_v2();

    //如果不加类型注解, 编译器会将v的类型推导为: std::vec::Vec<std::boxed::Box<Human>>
    // let v = vec![ 
    //     Box::new(Human), 
    //     Box::new(Dog) ,
    // ]; 


    let v: std::vec::Vec<std::boxed::Box<(dyn Mammal)>>  = vec![ 
        Box::new(Human), 
        Box::new(Dog) ,
    ];

    live_multi(&v); //ok
    // live_multi(&v[..]); //ok

}

```





### 生命周期

生命周期(lifetime)是Rust最与众不同的功能.


Rust编译器有 借用检查器(borrow checker) , 它会比较作用域来判断所有的借用是否有效.

如果被引用的对象比它的引用者存在的时间更短, 那么引用无效.


```rust
fn longest(x : &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```


```rust

// 通过在函数签名中指定生命周期参数时，
// 我们并没有改变任何传入后返回的值的生命周期。
// 而是指出任何不遵守这个协议的传入值都将被借用检查器拒绝。

// 泛型生命周期 'a  的具体生命周期  等于  x, y 两者生命周期较小者的生命周期
// 因此
// 就能保证返回的引用值在 x 和 y 中较短的那个生命周期结束之前保持有效。
fn longest<'a>(x : &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为。


```rust
// 函数或方法的引用参数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn longest_v2<'a, 'b>(x: &'a str, y: &'a str)  -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// //the reference is valid for the lifetime `'c` as defined on the function body at 23:23
// //but the borrowed content is only valid for the lifetime `'b` as defined on the function body at 23:19
// fn longest_v3<'a, 'b, 'c>(x: &'a str, y: &'b str)  -> &'c str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }



struct Coin<'a> {
    symbol: &'a str,
}


fn precious<'a>(x: &'a Coin, y: &'a Coin) -> &'a Coin<'a>{

    /*
    //OK
    if x.symbol.len() > y.symbol.len() {
        x
    } else {
        y
    }
    */


    /*
    // OK
    if x.symbol.len() > y.symbol.len() {
        return x;
    }
    return y;
    */

    if x.symbol.len() > y.symbol.len() {
        x;   // ^ expected `()`, found `&Coin<'_>`
        println!("x is {}", x.symbol);
        // return x;  //OK
    }
    y
}



/*

// OK

struct  Singer<'a> {
    name: &'a str,
    musics: &'a Box<Vec<String>>,
}

impl <'a>Singer<'a> {
    fn show(&'a self, music_name: &'a str) -> &'a str {
        for item in self.musics.iter() {
            if item == music_name {
                return item;
            }
        }
        &self.musics[0]
    }
}
*/



// OK
struct  Singer<'a, 'b> {
    name: &'a str,
    musics: &'b Box<Vec<String>>,  //如果换成  Vec<str> 则编译不过, 因为 str需要编译时期确定大小?
}

impl <'a, 'b>Singer<'a, 'b> {
    fn show(&'a self, music_name: &'b str) -> &'a str {
        for item in self.musics.iter() {
            if item == music_name {
                return item;
            }
        }
        &self.musics[0]
    }
}

fn main() {

    let a = String::from("hello");
    let b = String::from("good");

    let r = longest(&a,  &b);
    println!("longest is : {}", r);


    let r = longest_v2(&a, &b);
    println!("longest is : {}", r);


    // let r = longest_v3(&a, &b);
    // println!("longest is : {}", r);

    // drop(a); //moved
    let cny = Coin {symbol: &a};
    let usd = Coin {symbol: "USD" };

    let p =  precious(&cny, &usd);
    println!("precious: {}", p.symbol);


    let bx  = Box::new(vec![
        "toooooo".to_string(),
        "shut".to_string(),
        "apple".to_string(),
        "he is ok".to_string(),
    ]);

    {
        let sg = Singer { name:"gous", musics: &bx};
        let name = "apple";
        let ret = sg.show( &name );
        println!("ret = {}", ret);
    }

    // println!("{:?}", bx);
    
}

```




### 生命周期省略规则

在一些特定的情况下不需要指定生命周期:

- The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so on.

- The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.

- The third rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.


- 第一条规则是每一个是引用的参数都有它自己的生命周期参数。换句话说就是，有一个引用参数的函数有一个生命周期参数：`fn foo<'a>(x: &'a i32)`，有两个引用参数的函数有两个不同的生命周期参数，`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`，依此类推。

- 第二条规则是如果只有一个输入生命周期参数，那么它被赋予给所有输出生命周期参数：`fn foo<'a>(x: &'a i32) -> &'a i32`。

- 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)(译者注： 这里涉及rust的面向对象参见17章), 那么 self 的生命周期赋予给所有输出生命周期参数。第三条规则使得方法更容易读写，因为只需更少的符号。




```rust
//如果只有一个输入生命周期参数，那么它被赋予给所有输出生命周期参数
// fn show_append_suffix(x: &mut String) ->  &str {
fn show_append_suffix<'a>(x: &'a mut String) ->  &'a str {
    // x + "suffix"
    x.push_str("_suffix");
    &x[..]
}


struct Bottle<'a> {
    label: &'a str,
    cap: f64,
}

impl<'a > Bottle<'a > {
    //等效: fn get_label<'b>(&'a self, prefix: &'b str) -> &'a str {
    fn get_label(&self, prefix: &str) -> &str {
        println!("prefix:{}, {}:{}", prefix, self.label, self.cap);
        self.label
    }


    // 如果是方法, self的生命周期将被赋给所有输出生命参数
    // fn get_label_v2<'b>(&'a self, prefix: &'b str) -> &'a str {
    /*fn get_label_v2(&self, prefix: &str) -> &str { //等效上面的写法, 编译不过!
        println!("prefix:{}, {}:{}", prefix, self.label, self.cap);
        prefix
    }*/


    // fn get_label_v3<'b>(&'a self, prefix: &'b str) -> &'a str {
    /*fn get_label_v3(&self, prefix: &str) -> &str { //等效于上面的写法, 编译不过!
        println!("prefix:{}, {}:{}", prefix, self.label, self.cap);
        if self.label.len() >  prefix.len() {
            self.label
        } else {
            prefix
        }
    }*/

    // 必须加生命周期注解, 'a 是 min('a, 'b),  即生命周期最短的那个
    // fn get_label_v4(&'a self, prefix: &'a str) -> &'a str {
    fn get_label_v4(&self, prefix: &'a str) -> &str {
        println!("prefix:{}, {}:{}", prefix, self.label, self.cap);
        if self.label.len() > prefix.len() {
            self.label 
        } else {
            prefix
        }
    }

    // fn get_longest(&self, x: &str, y: &str) -> &str {
    fn get_longest(&self, x: &'a str, y: &'a str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}


fn main() {
    let mut x = String::from("good");
    let rf = show_append_suffix(&mut x);

    // 在任意给定时间，只能拥有"一个可变引用"或"任意数量的不可变引用之一"（而不是全部）。
    // println!("x = {}", &x); 
    println!("rf= {}", &rf);


    let bt = Bottle {label: "hope", cap:0.2234};
    let label  = bt.get_label("good");
    println!("label: {}", label);


    let longer = bt.get_longest("short", "longest");
    println!("longer: {}", longer);

}


```
