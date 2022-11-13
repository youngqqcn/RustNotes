
// Author: yqq
// Date: 2022-11-13 16:33:01
// Description: static


/*

'static 生命周期是可能的生命周期中最长的，它会在整个程序运行的时期中存在。
'static 生命周期也可被强制转换成一个更短的生命周期。有两种方式使变量拥有
'static 生命周期，它们都把数据保存在可执行文件的只读内存区：

- 使用 static 声明来产生常量（constant）。
- 产生一个拥有 &'static str 类型的 string 字面量。

*/

// 'static 生命周期的常量
static NUM: i32 = 18;

// 返回一个指向 `NUM` 的引用，该引用不取 `NUM` 的 `'static` 生命周期，
// 而是被强制转换成和输入参数的一样。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}


fn main() {
    {
        let ss = "ssssssssssssss";
        println!("static :{}", ss);
    }

    {
        let n = 9;

        // 将NUM 的生命周期强制转换成  n的生命周期
        let cc = coerce_static(&n);

        println!("{:?}", cc);
    }

    println!("{}", NUM);
}