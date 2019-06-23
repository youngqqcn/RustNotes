
//结构体



struct User {
    str_username : String,
    str_email : String,
    n_number : u64,
    b_isactive : bool,
}



fn main() {

    let mut user1 = User{
        str_username : String::from("yqq"),
        str_email : String::from("youngqqcn@163.com"),
        n_number : 9,
        b_isactive : true,
    };

    let mut user2 = User{
        str_email: String::from("youngqqcn@126.com"),
        str_username : user1.str_username.clone(), //不能使用  ..user1 直接复制
        ..user1   //剩下的字段使用  user1中的  因为是  i32 和 bool   
                    //属于Copy类型的, 不需要担心所有权的问题
    };


    user1.str_username = "yqq2".to_string();
    println!("================" );
    println!("name: {}", user1.str_username);
    println!("email: {}", user1.str_email);
    println!("number: {}", user1.n_number);
    println!("isactive: {}", user1.b_isactive);
    println!("================" );


    user2.str_username = "yqq3".to_string();
    println!("================" );
    println!("name: {}", user2.str_username);
    println!("email: {}", user2.str_email);
    println!("number: {}", user2.n_number);
    println!("isactive: {}", user2.b_isactive);
    println!("================" );

    let mut user3 = make_user(String::from("qwer"), String::from("234"));
    println!("================" );
    println!("name: {}", user3.str_username);
    println!("email: {}", user3.str_email);
    println!("number: {}", user3.n_number);
    println!("isactive: {}", user3.b_isactive);
    println!("================" );


    //使用没有命名字段的元组结构体  创建不同的类型
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let orgin = Point(0, 0, 0);


    println!("{}, {}, {}", black.0, black.1, black.2  );
    println!("{}, {}, {}", orgin.0, orgin.1, orgin.2  );



    //结构体数据的所有权问题
    struct Demo{
        username : &str,  // error[E0106]: missing lifetime specifier
        email : &str, //error[E0106]: missing lifetime specifier
        count : u64,
        active : bool,
    }

    let user5 = User{
        username : "somename",  //和C语言中的字符串常量存储位置不同
        email : "dsfd@qwer",
        active : true,
        count: 34,
    };

    println!("{}, {}, {}, {}", 
            user5.str_username, user5.str_email, 
            user5.n_number, user5.b_isactive )


}



fn make_user(str_username :String , str_email: String) -> User{
    User{
        str_email,  //变量与字段名同名时的字段初始化简写语法
        str_username,
        b_isactive :true,
        n_number : 1,
    }
}