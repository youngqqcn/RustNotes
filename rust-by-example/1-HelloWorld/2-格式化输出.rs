

// Author: yqq
// Date: 2022-11-02 22:08:09
// Description: 格式化输出

use std::fmt;

/*
format!      // 如上所述
write!       // 第一个参数是 &mut io::Write，目的地
writeln!     // 与 write 相同，但追加了一个换行符
print!       // 格式字符串被打印到标准输出
println!     // 与 print 相同，但追加了一个换行符
eprint!      // 格式字符串被打印到标准错误
eprintln!    // 与 eprint 相同，但追加了一个换行符
format_args! // 如下面所描述的。



输出指定的格式：

Binary	b 格式。
Debug	? 格式。
Display	空格式的格式 trait，{}。
LowerExp	e 格式。
LowerHex	x 格式。
Octal	o 格式。
Pointer	p 格式。
UpperExp	E 格式。
UpperHex	X 格式。
*/

fn main() {
    println!("hello world");

    // 通常情况下，`{}` 会被任意变量内容所替换，变量内容会转化成字符串。
    println!("{} day", 32);

    // 可以采用类似Python的格式化写法
    println!("{0}, {0}, {1} toooo {0}", 2, 1);

    // 使用命名参数
    println!("{name} {gender} {age}", name="yqq", gender="male", age=1);

    // 指定特定格式
    //  :b  表示二进制
    println!("{:b}", 10);

    // 文本对齐
    println!("{n:>width$}", n=1, width=10);

    // 一般用于输出错误
    eprintln!("{}", "errror");

    //===================作业
     // println! 会检查使用到的参数数量是否正确。
     println!("My name is {0}, {1} {0}", "Bond", "James");
     // 改正 ^ 补上漏掉的参数："James"

     // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    //  #[allow(dead_code)]
     #[derive(Debug)]
     struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0);
            Ok(())
        }
    }

     // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
     // 下面语句无法运行。
     println!("===> `{:?}`.", Structure(3));
     // 改正 ^ 注释掉此行。


     // 参考官方文档：https://rustwiki.org/zh-CN/std/fmt/#precision
     let pi = 3.141592;
     println!("Pi is roughly  {pi:.prec$}", pi=pi, prec=3);

}