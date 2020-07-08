


// 函数或方法的引用参数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn longest_v2<'a, 'b>(x: &'a str, y: &'a str)  -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// //the reference is valid for the lifetime `'c` as defined on the function body at 23:23
// //but the borrowed content is only valid for the lifetime `'b` as defined on the function body at 23:19
// fn longest_v3<'a, 'b, 'c>(x: &'a str, y: &'b str)  -> &'c str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }



struct Coin<'a> {
    symbol: &'a str,
}


fn precious<'a>(x: &'a Coin, y: &'a Coin) -> &'a Coin<'a>{

    /*
    //OK
    if x.symbol.len() > y.symbol.len() {
        x
    } else {
        y
    }
    */


    /*
    // OK
    if x.symbol.len() > y.symbol.len() {
        return x;
    }
    return y;
    */

    if x.symbol.len() > y.symbol.len() {
        x;   // ^ expected `()`, found `&Coin<'_>`
        println!("x is {}", x.symbol);
        // return x;  //OK
    }
    y
}



/*

// OK

struct  Singer<'a> {
    name: &'a str,
    musics: &'a Box<Vec<String>>,
}

impl <'a>Singer<'a> {
    fn show(&'a self, music_name: &'a str) -> &'a str {
        for item in self.musics.iter() {
            if item == music_name {
                return item;
            }
        }
        &self.musics[0]
    }
}
*/



// OK

struct  Singer<'a, 'b> {
    name: &'a str,
    musics: &'b Box<Vec<String>>,
}

impl <'a, 'b>Singer<'a, 'b> {
    fn show(&'a self, music_name: &'b str) -> &'a str {
        for item in self.musics.iter() {
            if item == music_name {
                return item;
            }
        }
        &self.musics[0]
    }
}

fn main() {

    let a = String::from("hello");
    let b = String::from("good");

    let r = longest(&a,  &b);
    println!("longest is : {}", r);


    let r = longest_v2(&a, &b);
    println!("longest is : {}", r);


    // let r = longest_v3(&a, &b);
    // println!("longest is : {}", r);

    // drop(a); //moved
    let cny = Coin {symbol: &a};
    let usd = Coin {symbol: "USD" };

    let p =  precious(&cny, &usd);
    println!("precious: {}", p.symbol);


    let bx  = Box::new(vec![
        "toooooo".to_string(),
        "shut".to_string(),
        "apple".to_string(),
        "he is ok".to_string(),
    ]);

    {
        let sg = Singer { name:"gous", musics: &bx};
        let name = "apple";
        let ret = sg.show( &name );
        println!("ret = {}", ret);
    }

    // println!("{:?}", bx);
    
    
}