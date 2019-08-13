

/*
fn main() {
    let mut v : Vec<i32> = Vec::new();

    let v2  = vec![1, 3, 4];



    v.push(5);
    v.push(6);
    v.push(7);

    println!("v[3] = {}", v[2]);




    let third = &v[2];
    println!("third: {}", third);



    //get 获取超出数组范围的值得时候  返回 None
    // []  数组越界时 会导致 程序崩溃
    match v.get(100){
        Some(third) => println!("{}", third ),
        None => println!("none" ),
    }

    if let Some(third) = v.get(2){
        println!("{}", third);
    }else{
        println!("{}", "non");
    }



    /*
    let v3 = vec![1, 2, 3, 4, 5];

    let not_exist = &v3[100];
    let not_exist = v3.get(100);
    */

    



    
    // println!("Hello, world!");
}
*/



/*

//所有权和借用规则

fn main(){

    let mut v = vec![1, 2, 3, 4, 6];

    //let first = v[0]; //值
    let first = v[0]; //引用

    v.push(999);

    println!("the first element is :{}", first );

}

// 为什么第一个元素的引用会关心 vector 结尾的变化？不能这么做的原因是由于 
// vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有所
// 有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
// 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。




*/





/*
//遍历vector
fn main(){

    let v = vec![100, 1123, 99];

    // for i in &v{ //不可变引用
    for i in v{ //值
        println!("{}", i);
    }


    for i in &mut v{
        *i += 50;
    }

}

for i in v{ //值
    |              - value moved here
...
102 |     for i in &mut v{
    |              ^^^^^^ value borrowed here after move


*/


/*
//遍历vector
fn main(){

    let v = vec![100, 1123, 99];

    // for i in &v{ //不可变引用
    for i in &v{ //值
        println!("{}", i);
    }
}
*/


/*
fn main(){

    let mut v = vec![10 ,  99,  300];

    for i in &mut v{
        *i += 50;
    }


    for i in &v{
        println!("{}", i);
    }

}
*/















