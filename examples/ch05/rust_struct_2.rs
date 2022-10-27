
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //关联方法, 在C++中叫静态方法
    fn name() -> String {
        String::from("Rectangle")
    }
}

impl Rectangle {
    fn get_width(&self) -> u32 {
        self.width
    }
}


fn main() {
    let rect = Rectangle { 
        width: 100,
        height: 100,
    };

    println!("面积: {}", rect.area());

    println!("name: {}", Rectangle::name());

    println!("width:{}", rect.get_width());
}