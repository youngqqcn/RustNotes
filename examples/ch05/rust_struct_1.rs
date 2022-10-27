
struct User {
    username: String,
    email: String,
}

/*
struct User {
    username: &str, //结构体中使用引用, 需要考虑引用的生命周期
    email: &str,
}
*/


struct Color(u8, u8, u8); //元组结构体
struct Point3D(f64, f64, f64);

struct CanFly; //没有数据字段, 类单元结构体


fn build_user(email: String, username: String) -> User {
    // 变量与字段同名时, 初始化可以简写(需要写出字段名称)
    User{
        username,
        email,
    }
}

fn main() {

    // let usr = User {"yqq", "1234@gmail.com" }; //ERROR

    let usr = build_user(String::from("yqq"), 
            String::from("123@gmail.com"));

    let usr2 = User {
        //只能用String, 不能用字符串字面值(&str), 因为生命周期的问题
        username: String::from( "yqq" ), 
        email: String::from( "234@gmail.com" )
    };

    let black = Color(0, 0, 0);

    let p = Point3D(1.23, -234.12, 992.999);
}