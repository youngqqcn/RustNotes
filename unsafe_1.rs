


fn main() {



    let mut num = 5;

    // let ref1 = &num;
    // let ref2 = &mut num;
    // println!("{}", *ref1);
    // *ref2 = 2222;
    // println!("{}", *ref2);

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = &mut num as *mut i32;

    unsafe {
        println!("r1 is : {}", *r1);

        *r2 = 999;
        println!("r1 is : {}", *r2);

        *r3 = 444;
        println!("r3 is : {}", *r3);
    }




    {

        let mut v = vec![1, 2, 3, 4, 5];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        println!("a = {:?}", a);
        println!("b = {:?}", b);



        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();

            assert!(mid <= len);


            use std::slice;

            //本质上借用 slice 的不同部分是可以的，因为结果两个 slice 不会重叠，
            //不过 Rust 还没有智能到能够理解这些。
            // (&mut slice[..mid], &mut slice[mid..])

            let ptr = slice.as_mut_ptr();
            unsafe {
                (slice::from_raw_parts_mut(ptr, mid),  
                slice::from_raw_parts_mut(ptr.add(mid), len - mid))
            }
        }


    }



}