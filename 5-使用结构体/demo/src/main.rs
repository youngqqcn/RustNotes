

/*
//计算  rectangle  的面积
fn main() {

    let width : f32 = 10.0;
    let height : f32 = 99.0;

    println!("area is {:.2}", clac_area(width, height));

    println!("Hello, world!");
}


fn clac_area(width : f32, height : f32) -> f32{
    width * height
}
*/


/*
fn main(){
    let rect1 = (30, 55);

    println!("area is : {}", clac_area(rect1));
}

fn clac_area(dim : (u32, u32)) -> u32{
    dim.0 * dim.1
}
*/


/*
#[derive(Debug)]
struct Rect{
    width : u32,
    height : u32,
}


fn main(){
    let rect  =  Rect{width: 30,  height : 90};
    //println!("area : {}", calc_area(rect));
    println!("area : {}", calc_area(&rect));  //注意所有权的借用问题
    println!("width : {}, height: {}", rect.width, rect.height);


    // println!("{}", rect); //直接打印rect
    println!("{:?}", rect); //直接打印rect
}

//fn calc_area(rect : Rect)->u32{
fn calc_area(rect : &Rect)->u32{
    rect.height  * rect.width
}
*/


#[derive(Debug)]
struct Rect{
    width : u32,
    height : u32,
}

//实现 Rect 的 area 方法    //类似  golang 中的面向对象方法 
impl Rect{
    fn calc_area(&self)->u32{   //类似  python的 self
        self.width * self.height  
    }

    fn can_hold(&self , other : &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    //关联方法 相当于C++中的静态方法
    fn make_square(size : u32) -> Rect{
        Rect{width : size , height : size}
    }
}


impl Rect{

    fn get_width( &self ) -> u32{
        self.width
    }

}


fn main(){
    let rect  =  Rect{width: 30,  height : 90};
    //println!("area : {}", calc_area(rect));
    println!("area : {}", rect.calc_area());  //注意所有权的借用问题
    println!("width : {}, height: {}", rect.width, rect.height);
    println!("{:?}", rect); //直接打印rect

    let rect1  =  Rect{width: 20,  height : 80};
    if  rect.can_hold( &rect1 ) {
        println!("can hold");
    }else{
        println!("can not hold");
    }


    //调用关联方法
    let rect3 =  Rect::make_square(9);
    println!("the area of rect3 is : {}", rect3.calc_area() );


    //多个 impl
    println!("{}",  rect3.get_width());

}



