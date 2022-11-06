// Author: yqq
// Date: 2022-11-06 18:44:27
// Description: if let


// 为什么用 if let ， 因为  match 有时候太啰嗦，不优雅

/*

// 将 `optional` 定为 `Option<i32>` 类型
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
        // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
    },
    _ => {},
    // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
};


*/


enum Foo {
    Bar,
    Baz,
    Qux(i32)
}

fn main() {
    // 全部都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    // 语句块（`{}`）
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }


    if let Some(x) = letter {
        println!("matched is {}", x)
    } else {
        println!("not match")
    }

    let i_like_letters = true;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件的值为 false。于是以下是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };


    // 使用 if let 匹配enum类型

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(55);

    // if Foo::Bar == a {
    //     // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
    //     println!("a is foobar");
    // }

    if let Foo::Bar = a {
        println!("barrrrrrrr");
    }

    if let Foo::Baz = b {
        println!("bazzzzzzzzzzzzz");
    }

    if let Foo::Qux(x) = c {
        println!("x is {}", x);
    }

}