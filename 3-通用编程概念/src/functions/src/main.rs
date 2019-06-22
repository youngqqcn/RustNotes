/*
fn main() {

    //Rust 代码中的函数和变量名使用 snake case 规范风格。
    //在 snake case 中，所有字母都是小写并使用下划线分隔单词。


    //调用foo
    foo();

    foo2(997);




    //println!("Hello, world!");
}




//Rust中不关心 函数定义在何处
//只要定义了就行
fn foo(){
    println!("foooooooooooooooooooo")
}


fn foo2( x : i32){
    println!(" value is : {}", x)
}
*/




/*
语句没有返回值
表达式有返回值


函数本身是语句
函数调用时表达式

宏本身是语句
宏调用时表达式

代码块 即用{}包起来的 是表达式

*/

/*
fn main(){

    //let x = (let y = 9);   // error

    let x = 5;
    let y = {
        let x = 3;
        x + 10      //表达式结尾没有分号,   如果加上分号,  就变成了语句
    };

    /*
    let m = {
        let x = 3;
        x + 10;      //表达式结尾没有分号,   如果加上分号,  就变成了语句
    };
    println!("value of m is : {}", m);
    */

    println!("value of y is : {}", y);
}
*/



fn main(){

    let x = get_value(1);

    println!("get_value() result: {}", x);
}


fn get_value(x : i32) ->i32 {
    //return 5; //ok
    return x + 10;
    //5  //ok
    // x  + 10  //ok
    // x + 10; //error
}




