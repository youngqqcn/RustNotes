// Author: yqq
// Date: 2022-11-14 22:30:36
// Description: Drop

// Drop trait 只有一个方法：drop，当对象离开作用域时会自动调用该 方法。
// Drop trait 的主要作用是释放实现者的实例拥有的资源。

// Box，Vec，String，File，以及 Process 是一些实现了 Drop trait 来释放 资源的类型。
// Drop trait 也可以为任何自定义数据类型手动实现。

struct Droppable {
    name: &'static str,
}


impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {

    let _a = Droppable { name: "a" };

    {// block A


        let _b = Droppable { name : "b" };

        { // block B

            let _c = Droppable { name : "c" };
            let _c = Droppable { name : "d"};

            println!("exitng block B ");
        }

        println!("Just Exited block B");

        println!("Existing block A");
    }
    println!("just exited block A");

    // 手动调用？？
    drop(_a);


    println!("=========");

    // `_a` *不会*在这里再次销毁，因为它已经被（手动）销毁。
}