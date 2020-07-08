


fn main() {

    let x = 1; // let 不可反驳模式


    match x {  //因为match的匹配是穷尽的, 即匹配所有可能的值, 所以是不可反驳
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    };



    //以下这段代码, 会打印出什么?
    {
        let x = Some(5);
        let y = 10;
    
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }
    
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }


    {
        let x  = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }


    {
        let x = 9;
        match x {
            1..=5 => println!("one through five"),
            // 6..10 => println!("six to 10"),
            _ => println!("something else"),
        }
    }



    {
        let x = '5';

        match x {
            'a' ..='j' => println!("a..j"),
            'k' ..='z' => println!("k..z"),
            _ => println!("something else"),
        }
    }


    // 解构结构体
    {
        struct Point {
            x: i32,
            y: i32,
        }
        
        
        let p = Point { x: 0, y: 7 };
    
        // let Point { x: a, y: b } = p;  //解构结构体
        let Point { x, y } = p;  //解构结构体
        // println!("a  = {}", a);
        // println!("b  = {}", b);
        
        println!("x  = {}", x);
        println!("y  = {}", y);
    }

    {
        struct Point {
            x: i32,
            y: i32,
        }
        

        let p = Point{ x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x at {}", x),
            Point { x: 0, y} => println!("On the y at {}", y),
            Point { x, y } => println!("other {}, {}", x, y),
        }
    }


    // 解构枚举
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32,i32),
        }


        // let msg = Message::ChangeColor(0xff, 0xff, 0xff); //black
        // let msg = Message::Move{x:99, y:0};
        // let msg = Message::Write(String::from("this is string"));
        let msg = Message::Quit{};

        match msg {
            Message::Quit => {
                println!("quit msg");
            }
            Message::Move{x, y} => {
                println!("move to {}, {}", x, y);
            }
            Message::Write(text) => println!("text msg: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("change color to {},{},{}", r, g, b);
            }
        }
    }



    //解构嵌套的结构体和枚举
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(Color),
        }


        let msg = Message::ChangeColor(Color::Hsv(0, 55, 123));

        match msg {
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("change color to h:{}, s:{}, v:{}", h, s, v);
            },
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("change color to r:{}, g:{}, b:{}", r, g, b);
            },
            _ => ()
        }


    }


    //解构结构体和元组
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let ((feet, inches, tmpstr), Point {x, y}) = ((3, 10, String::from("hello")), Point { x: 3, y: -10 });
        println!("feet: {} inches:{}", feet, inches );
        println!("x:{}, y:{}, tmpstr:{}", x, y, tmpstr);
    }



    // 忽略模式中的值

    {
        fn foo(_: i32, y: i32) { // 忽略 第一个参数
            println!("foo() y = {}", y);
        }

        foo(3, 4);
    }


    {
        // let mut setting_value = Some(5);
        let mut setting_value = None;
        let new_setting_value = Some(10);
        // let new_setting_value = None;

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite and existing customized value")
            },
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);
    }


    // 可以在一个模式中的多处使用下划线来忽略特定值
    {

        let numbers  = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth)  => {
                println!("some numbmer: {}, {}, {}", first, third, fifth);
            },
        }

    }

    // 通过在变量名字前加一个下划线作为开头, 来忽略未使用变量
    {
        let _x = 99;
        let y = 999;
    }


    {
        
        let s = Some(String::from("Hello!"));

        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
    }



    // 用 .. 忽略剩余值
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let orgin  = Point{ x: 1, y: 0, z: 0 };
        
        match orgin {
            Point{x, .. } => println!("x is {}", x),
        }



        let numbers = (1, 2, 3, 4);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }

    }



    // 匹配守卫  (match guard)
    {
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }


        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {}", n),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {}", x, y);
        
    }

    {
        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),  // x 为 4, 5 或 6     并且  y 为true
            _ => println!("no"),
        }
    }



    // @ 绑定
    {
        enum Message {
            Hello {id: i32 },
        }
        
        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {id: id_var @ 3..=7 } => {
                // 测试 Message::Hello 的 id 字段是否位于 3...7 范围内，
                // 同时也希望能将其值绑定到 id_variable 变量中以便此分支相关联的代码可以使用它。
                println!("=====1========= {}", id_var);
            },
            Message::Hello { id: 10..=12 } => {
                // id 字段的值可以是 10、11 或 12，不过这个模式的代码并不知情
                //也不能使用 id 字段中的值，因为没有将 id 值保存进一个变量。
                println!("===========2===========")
            },
            Message::Hello { id } => {
                println!("===========3=============");
            },
        }

    }




}