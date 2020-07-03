


//如果只有一个输入生命周期参数，那么它被赋予给所有输出生命周期参数
// fn show_append_suffix(x: &mut String) ->  &str {
fn show_append_suffix<'a>(x: &'a mut String) ->  &'a str {
    // x + "suffix"
    x.push_str("_suffix");
    &x[..]
}


struct Bottle<'a> {
    label: &'a str,
    cap: f64,
}

impl<'a > Bottle<'a > {
    //等效: fn get_label<'b>(&'a self, prefix: &'b str) -> &'a str {
    fn get_label(&self, prefix: &str) -> &str {
        println!("prefix:{}, {}:{}", prefix, self.label, self.cap);
        self.label
    }


    // 如果是方法, self的生命周期将被赋给所有输出生命参数
    // fn get_label_v2<'b>(&'a self, prefix: &'b str) -> &'a str {
    /*fn get_label_v2(&self, prefix: &str) -> &str { //等效上面的写法, 编译不过!
        println!("prefix:{}, {}:{}", prefix, self.label, self.cap);
        prefix
    }*/


    // fn get_label_v3<'b>(&'a self, prefix: &'b str) -> &'a str {
    /*fn get_label_v3(&self, prefix: &str) -> &str { //等效于上面的写法, 编译不过!
        println!("prefix:{}, {}:{}", prefix, self.label, self.cap);
        if self.label.len() >  prefix.len() {
            self.label
        } else {
            prefix
        }
    }*/

    // 必须加生命周期注解, 'a 是 min('a, 'b),  即生命周期最短的那个
    // fn get_label_v4(&'a self, prefix: &'a str) -> &'a str {
    fn get_label_v4(&self, prefix: &'a str) -> &str {
        println!("prefix:{}, {}:{}", prefix, self.label, self.cap);
        if self.label.len() > prefix.len() {
            self.label 
        } else {
            prefix
        }
    }

    // fn get_longest(&self, x: &str, y: &str) -> &str {
    fn get_longest(&self, x: &'a str, y: &'a str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}


fn main() {
    let mut x = String::from("good");
    let rf = show_append_suffix(&mut x);

    // 在任意给定时间，只能拥有"一个可变引用"或"任意数量的不可变引用之一"（而不是全部）。
    // println!("x = {}", &x); 
    println!("rf= {}", &rf);


    let bt = Bottle {label: "hope", cap:0.2234};
    let label  = bt.get_label("good");
    println!("label: {}", label);


    let longer = bt.get_longest("short", "longest");
    println!("longer: {}", longer);

}

