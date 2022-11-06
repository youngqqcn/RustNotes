// Author: yqq
// Date: 2022-11-06 18:59:46
// Description:  while let 使 循环匹配变得优雅


/*
fn main() {

    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 重复运行这个测试。
    loop {
        match optional {
            // 如果 `optional` 解构成功，就执行下面语句块。
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ 需要三层缩进！
            },
            // 当解构失败时退出循环：
            _ => { break; }
            // ^ 为什么必须写这样的语句呢？肯定有更优雅的处理方式！
        }
    }
}
*/



fn main() {

    let mut op = Some(0);

    // 优雅
    
    while let Some(x) = op {

        if x > 9 {
            println!("Greater than 9, quit!");
            break;
        } else {
            println!("`i` is `{:?}`. Try again.", x);
            op = Some(x + 1);
        }
    }
}