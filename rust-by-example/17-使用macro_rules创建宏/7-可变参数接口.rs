// Author: yqq
// Date: 2022-11-16 22:49:35
// Description: 可变参数接口


macro_rules! calculate {
    // 单个 `eval` 的模式
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // 递归地拆解多重的 `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}




fn main() {
    calculate! { // 妈妈快看，可变参数的 `calculate!`！
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}



// Rust的宏，真的太屌了！ 只有你想不到的，没有Rust宏做不到的！ 简直给Rust安装上翅膀！