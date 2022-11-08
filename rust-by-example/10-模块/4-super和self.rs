// Author: yqq
// Date: 2022-11-08 22:20:30
// Description: super 和 self

// 可以在路径中使用 super （父级）和 self（自身）关键字
// ，从而在访问项时消除 歧义，以及防止不必要的路径硬编码。


fn function() {
    println!("11111111111");
}

mod foooo {
    pub fn function() {
        println!(
            "22222222222"
        );
    }
}

mod my {
    fn function() {
        println!("3333333333333");
    }

    mod foooo {
        pub fn function() {
            println!(
                "44444444444444"
            );
        }
    }

    pub fn indirect_call() {
        println!("55555555555555");

        self::function(); // 内部
        function(); // 外部

        self::foooo::function();

        super::function(); // 外部

        {
            use crate::foooo::function as root_func;
            root_func();
        }
    }


}


fn main() {
    my::indirect_call();
}