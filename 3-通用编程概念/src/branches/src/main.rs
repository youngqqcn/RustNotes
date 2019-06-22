


fn main() {

    let n_test = 5;

    // if n_test <= 5{
    //     println!(" n_test <= 5");
    // }else{
    //     println!("n_test > 5")
    // }


    /*
    if n_test < 5 {
        println!("n < 5" );
    }else if(n_test == 5){
        println!("n == 5" );
    }else if (n_test > 5){
        println!("n > 5");
    }
    */


    /*
    if n_test{  // error 只能使用 bool类型
        println!("true........" );
    }
    */

    //相当于  c++中的  三目运算符
    let x = if true{5} else {9};
    //let x = if true{5} else { "666" }; // errror , 类型不匹配
    println!("{}", x);




    //循环
    let mut i = 0;
    let mut usum = 0;
    loop{
        if !(i < 10){
            break;
        }
        usum += 1;

        i += 1;
    };
    println!("sum is : {}", usum );


    //从循环返回
    let mut i = 0;
    let usum = loop{

        if !(i < 10){
            break i * 10   //ok
            // break i * 10;  //ok
        }
        usum += 1;

        i += 1;
    };

    println!("sum is : {}", usum );


    //while
    let mut i = 0;
    while  i < 100{
        i += 1;
        println!("{}", i);
    }


    //for

    
    //遍历数组
    let arr = [1, 2, 3, 4, 5, 6, 8];

    //使用while循环遍历数组
    let mut i = 0;
    while i < arr.len() {
        println!("{}", arr[i]);
        i += 1;
    }

    //使用for循环遍历数组
    for item in arr.iter(){
        println!("{}", item);
    }


    for item in (1..4).rev(){
        println!("{}!", item);
    }
    println!("LIFTOFF!")




    // println!("Hello, world!");
}





