use std::intrinsics::type_name;

fn zip_test(){

    let v1 = vec![1, 2, 3];
    let v2 = vec![5, 6];

    let mut it = v1.iter().zip(v2);
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());


    let unknow = ((1, 2),(3,4));
    let unknow2 = (1, 9);   //元组类型
    let unknow4 = (13, 9);
}



fn main() {
    zip_test();


    let v1 = vec![1, 3, 5, 8, 9];


    let it = v1.iter();


    for val in it{
        println!("{}", val);
    }



    let v2 = vec![1, 9, 10];

    let mut it2 = v2.iter();
    let total : u32 = it2.sum();  //moved here
    println!("{}", total);
//    println!("{:?}", it2.next());



    //let vtmp : Vec<i32> = v1.iter().map(|x| x + 3).collect();
    let vtmp : Vec<_> = v1.iter().map(|x| x + 3).collect();
    println!("{:?}", vtmp);




    println!("Hello, world!");
}


























