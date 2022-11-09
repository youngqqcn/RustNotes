// Author: yqq
// Date: 2022-11-09 22:34:51
// Description: dead_code


fn used_function() {
}

// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {
}

#[allow(dead_code)]
fn noisy_unused_function() {
}
// 改正 ^ 增加一个属性来消除警告

fn main() {
    used_function();
}
