
fn main() {

    //无限循环
    /*
    loop {
        println!("ok......");
    }
    */

    // 从循环返回(可以带上返回值), 这一点与其他语言不通
    let mut n = 0;
    let result = loop {
        n += 1;
        if n == 10{
            break n * 2;  // 将 n * 2 作为循环的返回值
        }
    };
    println!("result = {}", result);



    /*
    let mut n = 0;
    let result = while true {
        n += 1;
        if n == 10{
            // can only break with a value inside `loop` or breakable block
            break n * 2;   // 编译错误
        } 
    }
    println!("result = {}", result);
    */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result); 


    // for 循环

    let mut sum = 0;
    for i in 0..100 {
        sum += i;
    }
    println!("sum = {}", sum);


    let a  = [1, 2, 3, 4, 5, 6];

    for it in a.iter() {
        //it 是(引用)迭代器

        // println!("the value is {}", *it); //OK, 手动解引用
        println!("the value is {}", it); //ok, Rust 会自动解引用
    }

    // for it in a.reverse() {
    for it in (1..14).rev(){
        println!("the value is {}", it); //ok, Rust 会自动解引用
    }

}