// Author: yqq
// Date: 2022-11-13 10:35:07
// Description: 别名使用

// 数据可以多次不可变借用，但是在不可变借用的同时，原始数据不能使用可变借用。
// 或者说，同一时间内只允许一次可变借用。仅当最后一次使用可变引用之后，
// 原始数据才可以再次借用。


// struct Point { x: i32, y: i32, z: i32 };


fn main() {

    {

        let b = 1;
        let a = b;

        println!("a = {} ", a);
        println!("b = {}", b);
        // println!("*a = {}", *a); // error:  type `{integer}` cannot be dereferenced

        let c = 2;
        // a = c; // error: cannot assign twice to immutable variable
    }

    {
        let b = 2;
        let a = &b;
        println!("a = {}, b = {}", a, b);
        println!("a = {}, b = {}", *a, b);

        // *a = 99; // `a` is a `&` reference, so the data it refers to cannot be written
    }


    {
        let b = 2;
        // let mut a = &b;
        let mut a = &b;
        println!("&&&& a = {}, b = {}", a, b);
        println!("a = {}, b = {}", *a, b);
        let c = 3;
        a = &c;
        println!("a = {}", a);
    }



    // {
    //     let b = 2;
    //     let a = &mut b; // error:  cannot borrow `b` as mutable, as it is not declared as mutable
    //     println!("a = {}, b = {}", a, b);
    //     // println!("a = {}, b = {}", *a, b);

    //     // *a = 99; // `a` is a `&` reference, so the data it refers to cannot be written
    // }
    {
        let mut b = 1;
        let a = &mut b;
        println!("xxx===> a = {}", a);
        // println!("xxx===> a = {}, b = {}", a, b); // error
        // println!("b = {}", b); //error
        *a = 99;
        // println!("xxx===> a = {}, b = {}", a, b); // error
        println!("xxx===> a = {}", a);
        println!("b = {}", b);

        let mut c = 1;
        // a = &mut c;  // error, cannot assign twice to immutable variable `a`
    }

    {
        let b = 1;
        let mut a = b; // 将b的值进行了拷贝
        println!("=jj==> a = {}, b = {}", a, b);
        a = 2;
        println!("===> a = {}, b = {}", a, b);
    }


    {
        let b = 1;
        let &(mut a) = &b;
        println!("+++ a = {}, b = {}", a, b);
        a  = 999;
        println!("+++ a = {}, b = {}", a, b);
        // println!("+++ a = {}, b = {}", *a, b); // type `{integer}` cannot be dereferenced
    }

    {
        let b = 1;
        // let ref mut a  = b;  // 错，同 let a = &mut b;
        let mut c = 1;
        let ref mut a = c;
        *a = 99;
        // println!("a = {}, c = {}", *a, c); // c已经被可变借用了，不能再使用
        println!("a = {}", *a);
        println!("c = {}", c);

        let mut d = 777;
        a = &mut d;
        println!("a = {}", a);
    }

    {
        let mut c = 1;
        let a = &mut c;
        *a = 99;
        println!("a = {}", *a);

        let mut d = 33;
        // a = &mut d; // error
    }

    {
        let mut c = 1;
        let mut a = &mut c;
        *a = 99;
        println!("a = {}", *a);

        let mut d = 33;
        a = &mut d; // ok
        println!("a = {}", *a);
    }
}

// let b = 1;
//
// 思考下面的情况， a代表什么？
// let a = b;   // a是不可变变量
// let a = &b;  // a是不可变应用
// let a = &mut b; // 错误，因为b是不可变变量，不能进行可变借用
// let mut a = b;  // 将b的值拷贝到a， a是可变变量
// let &(mut a) = &b;  // 同 let mut a = b; a是普通可变整数类型
// let ref a = b;  // 同 let a = &b;
// let mut c = 1; let ref mut a = c;   //  同 let &mut a = c;