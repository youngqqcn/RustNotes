





fn main() {

    let v1 = vec![1, 23, 99];
    let mut iter = v1.iter();                            


    // it = iter.next()  // 获取不可变引用

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&23));
    assert_eq!(iter.next(), Some(&99));
    assert_eq!(iter.next(), None);



    let mut v2 : Vec<u32> = Vec::new();
    v2.push(99);
    v2.push(199);
    v2.push(299);
    v2.push(399);

    let mut mutit = v2.iter_mut(); // 可以修改元素的值
    let item = mutit.next();
    *item.unwrap() += 2;   //仅对第一个元素进行  加 2
    
    for it in v2.iter_mut() {
        *it += 2;
    }
    println!("{:?}", v2);



    let v3 = vec![1, 2 ,3];
    let v3_it = v3.iter();
    let sum: i32 = v3_it.sum();
    // let sum: u8 = v3_it.sum();
    assert_eq!(sum , 6);



    let v4: Vec<i32> = vec![1, 2, 3];
    let v4.iter().map(|x| x + 1);


    // Vec<_>  是啥意思?   _ 是占位符,  
    // https://stackoverflow.com/questions/34363984/what-is-vec
    //  v5 是 Vec<T> , 其中 T 类型交由编译自动推导
    let v5: Vec<_>  = v4.iter().map(|x| x + 1).collect();
    // assert_eq!(v5, vec![2, 3, 4]);
    

    

    
}