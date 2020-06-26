



/*
fn main() {


    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r : {}", r);

}
*/


/*
fn main() {
    let str1 = String::from("abcdefg");
    
    // let str2 = "xyz";
    // let result = longest(str1.as_str(), str2);
    // println!("the longest string is {}", result);

    {
        let str3 = String::from("this");
        let ret = longest(str3.as_str(), str1.as_str());
        println!("The longest one: {}", ret);
    }
}
*/



fn main() {
    let str1 = String::from("abcdefg");
    

    let ret;
    {
        let str3 = String::from("this");

        // rust 编译器 根据   str3  str1  生命周期较短这 决定  
        ret = longest(str3.as_str(), str1.as_str());
        //println!("The longest one: {}", ret); // ok
    }
    println!("The longest one: {}", ret); // error:  borrowed value does not live long enough
}



/*
fn longest(x : &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/


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