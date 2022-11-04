// Author: yqq
// Date: 2022-11-04 21:59:06
// Description: use 的使用

enum Status {
    Begin,
    Pending,
    Stop,
}


enum Work {
    Civilian,
    Soldier,
    Teacher,
    Doctor
}



fn main() {
    use Status::{Begin, Stop};

    // 使用Work内部所有
    use Work::*;


    let status = Stop;

    match status {
        Begin => println!("Beginnnnnnnnnnnn"),
        Pending => println!("Pendingnnnnnnnnnnnn"),
        Stop => println!("Stoppppppppppppppp"),
    }

    let work = Teacher;

    match work {
        Teacher => println!("Hi, teacher"),
        _ => println!("other"),
    }

    println!("hello world");
}