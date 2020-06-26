


fn main() {

    let flag : bool = true;

    let a_float : f64 = 1.234234293;
    let number = 5323i32;


    let mut infered_type  = 123;
    infered_type = 1224234234234i64;



    let mut mutable = 234;
    mutable = 23432234;

    //  expected integer, found `bool`
    // mutable = true;

    //覆盖
    // Variables can be overwritten with shadowing.
    let mutable = true;
}