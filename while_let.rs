


fn main() {


    let mut statck = Vec::new();

    statck.push(1);
    statck.push(2);
    statck.push(3);
    statck.push(4);

    // while 循环只要 pop 返回 Some 就会一直运行其块中的代码。一旦其返回 None，while 循环停止。
    while let Some(n) = statck.pop() {
        println!("number: {}", n);
    }

}