// Author: yqq
// Date: 2022-11-21 22:46:16
// Description:

// Rust 中有两种字符串类型：String 和 &str。

// String   是  Vec<u8>,  堆分配
//  &str  是 &[u8]

fn main() {
    let pangram: &str = "hello world good";
    println!("{}", pangram);

    for w in pangram.split_whitespace().rev() {
        println!("> {}", w);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.dedup();

    let mut string = String::new();

    for c in chars {
        string.push(c);
        string.push_str(",xxx")
    }
    println!("{}", string);

    let chars_to_trim: &[char] = &['x', ','];
    let trimemed_str: &str = string.trim_matches(chars_to_trim);
    println!("xxxxxxxx: {}", trimemed_str);

    let alice = String::from("i like dogs");
    let bod: String = alice.replace("dogs", "food");

    println!("{}", alice);
    println!("{}", bod);

    // 通过转义，可以用十六进制值来表示字节。
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // 也可以使用 Unicode 码位表示。
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                          can span multiple lines.
                          The linebreak and indentation here ->\
                          <- can be escaped too!";
    println!("{}", long_string);


    // 注意这并不是一个 &str
    let bytestring: &[u8; 20] = b"this is a bytestring";

    // 字节串没有实现 Display，所以它们的打印功能有些受限
    println!("A bytestring: {:?}", bytestring);

    // 字节串可以使用单字节的转义字符...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但不能使用 Unicode 转义字符
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // 原始字节串和原始字符串的写法一样
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 把字节串转换为 &str 可能失败
    if let Ok(my_str) = std::str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;
    // println!("{}", quotes);

    // 字节串可以不使用 UTF-8 编码
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // SHIFT-JIS 编码的 "ようこそ"

    // 但这样的话它们就无法转换成 &str 了
    match std::str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}
