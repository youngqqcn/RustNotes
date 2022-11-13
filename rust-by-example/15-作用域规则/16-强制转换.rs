// Author: yqq
// Date: 2022-11-13 16:23:59
// Description: 强制转换


fn multiply(first: &i32, second: &i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` 读作生命周期 `'a` 至少和 `'b` 一样长。
// 在这里我们我们接受了一个 `&'a i32` 类型并返回一个 `&'b i32` 类型，这是
// 强制转换得到的结果。
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

// 一个较长的生命周期可以强制转成一个较短的生命周期，
// 使它在一个通常情况下不能工作的作用域内也能正常工作



fn largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    }else {
        y
    }
}

fn main() {

    let f = 2;

    {
        let s = 3;
        println!("{}", multiply(&f, &s));

        println!("{}", choose_first(&f, &s));

        println!("{}", largest(&f, &s));
    }

}