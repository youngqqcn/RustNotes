// Author: yqq
// Date: 2022-11-09 22:48:26
// Description: 函数

struct A;
struct S(A);
struct SGen<T>(T);


fn foo(_s: S) {
}

// A 是  struct A
fn foo2(_s: SGen<A>) {
}

fn foo3(_s: SGen<i32> ) {
}

// 泛型
fn gen_foo(_s: SGen<T>) {
}

fn main() {
    // 使用非泛型函数
    reg_fn(S(A));          // 具体类型。
    gen_spec_t(SGen(A));   // 隐式地指定类型参数 `A`。
    gen_spec_i32(SGen(6)); // 隐式地指定类型参数 `i32`。


    gen_foo::<char>(SGen('aaa'));


    gen_foo(SGen('c'));
}