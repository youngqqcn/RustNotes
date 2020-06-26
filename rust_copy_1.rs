
fn main() {

    //整型 数组是Copy的
    let x = 5;
    let y = x; 
    println!("x = {}, y = {}", x, y);


    //仅包含Copy的类型栈上数组是 Copy 的
    let a = [1, 2, 3]; // 栈上数组
    let b = a;
    println!("a = {:?}", a);
    println!("b = {:?}", b);


    // 堆上数组不是 Copy的
    let sa = vec![1, 2, 3]; //堆上数组  Vec<i32> 
    let sb = sa; //moved
    // println!("sa = {:?}", sa);  //ERROR
    println!("sb = {:?}", sb);

    // 包含堆上数据 栈上数组不是 Copy的
    let ss = [vec![1,2,3], vec![4, 5, 6]];
    let ss2 = ss; // moved
    // println!("{:?}", ss);  //ERROR
    println!("{:?}", ss2);



    // 布尔类型  是 Copy的
    let bl: bool = false;
    let cl = bl;
    println!("bl = {}, cl = {}", bl, cl); // OK

    
    //浮点型 f32 和 f64  是Copy的
    let f: f32 = 0.234f32;
    let f2 = f;
    println!("f = {}, f2 = {}", f, f2); //OK

    //字符型 char  是 Copy的
    let ch = '中';
    let ch2 = ch;
    println!("ch = {}, ch2 = {}", ch, ch2); //OK


    //仅包含 Copy类型的 元组
    let tp = (1, 2.4, '国', true, "good", [1, 2, 3]);
    let tp2  = tp;
    println!("tp = {:?}", tp); // OK
    println!("tp2 = {:?}", tp2); //OK

}