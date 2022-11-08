// Author: yqq
// Date: 2022-11-08 22:16:50
// Description: use声明

// 重命名
use deeply::nested::function as other_function;



fn function() {
    println!("calllllllllll");
}


mod deeply {
    pub mod nested {
        pub fn function() {
            println!("xxxxxxxxxxxxxxxx");
        }
    }
}


fn main() {

    other_function();


    {
        use deeply::nested::function;
        function(); // 外部的function被遮蔽

        // println!("xxxxxxx");
    }

    function();
}