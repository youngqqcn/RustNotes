// Author: yqq
// Date: 2022-11-13 10:10:08
// Description: 可变应用


// 注意： 这里的Clone 和 Copy trait
#[derive(Clone, Copy)]
// #[derive(Clone )]   // error  move occurs because `immutablebook` has type `Book`, which does not implement the `Copy` trait
// #[derive(Copy)] // error , the trait `Clone` is not implemented for `Book`
struct Book {

    // `&'static str` 是一个对分配在只读内存区的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("{}, {}", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 9999;
    println!(" updated book: {}, {}", book.title, book.year);
}

fn main() {

    let immutablebook = Book {
        author: "ddddd",
        title: "GGGGG",
        year: 1999,
    };



    // 这里不是将immutablebook移动到mutbook，因为 Book有Clone和Copy trait
    // 这里是拷贝(Clone),
    //
    // 如果 Book 没有 Clone 和 Copy  trait, 那么， 这里是移动immutablebook
    let mut mutbook = immutablebook;

    borrow_book(&immutablebook);

    borrow_book(&mutbook);


    new_edition(&mut mutbook);


    borrow_book(&immutablebook); //  可以看到immutablebook的内容并没有改变


    // 如果 Book 没有 Clone 和 Copy  trait,
}