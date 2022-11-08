// Author: yqq
// Date: 2022-11-08 22:08:18
// Description:结构体可见性

mod my {

    pub struct OpenBox<T> {
        pub content: T, // 公有字段
    }

    pub struct ClosedBox<T> {
        content: T, // 私有, 不能直接访问
    }

    impl<T> ClosedBox<T> {
        // 公有函数
        pub fn new(c: T) -> ClosedBox<T> {
            ClosedBox {
                content: c
            }
        }
    }

}

fn main() {

    let open_box = my::OpenBox{ content: "666"};
    println!("the open box contains: {}", open_box.content);

    // 不能直接访问私有字段
    // let closed_box = my::ClosedBox{content: "xxx"};

    let closed_box = my::ClosedBox::new("xxx");
    // println!("{}", closed_box.content); // 私有字段不可见

}
