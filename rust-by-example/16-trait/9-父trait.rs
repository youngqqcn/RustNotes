// Author: yqq
// Date: 2022-11-15 22:28:57
// Description: 父trait

// Rust 没有“继承”，但是您可以将一个 trait 定义为另一个 trait 的超集（即父 trait）

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait ComputerScience: Programmer + Student {
    fn git_username(&self) -> String;
}


fn comp_sci_student_greeting(student: &dyn ComputerScience) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}


fn main() {
    println!("hello world");
}