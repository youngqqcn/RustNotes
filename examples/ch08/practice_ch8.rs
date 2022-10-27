
use std::collections::HashMap;

fn get_average_num(vct: &Vec<i32>) -> f64 {

    let mut sum = 0;

    for n in vct {
        sum += n;
    }

    let size = vct.len() as u32;

    let avr = f64::from(sum) / f64::from( size );
    avr
}


//获取中位数
fn get_middle_number(vct: &Vec<i32>) -> i32 {
    // vct.sort();
    let mut vct_cp = vct.clone();
    vct_cp.sort();

    let npos  = vct_cp.len() / 2;
    println!("npos: {}", npos);
    vct_cp[ npos ]
}

//获取众数
fn get_mode_number(vct : &Vec<i32>) -> i32 {

    let mut counter_map = HashMap::new();
    for num in vct {
        let count =  counter_map.entry(num).or_insert(0);
        *count += 1;
    }

    println!("counter_map: {:?}", counter_map);


    // // 1
    let mut max_tp = (0, 0);
    for (k, v) in counter_map{
        println!("{}, {}", k, v);
        if max_tp.1 < v {
            max_tp = (*k, v)
        }
    }

    max_tp.0
}



fn pig_latin(word : &String) -> String {

    //辅音字母
    let con_alpha = vec!['b','c','d','f','g','h','j','k','l',
                'm','n','p','q','r','s','t','v','w','x','y','z'
     ];

    //元音字母
    let univ_alpha = vec!['a', 'e', 'i', 'o', 'u'];


    fn in_set(ch : &char, vct: &Vec<char>) -> bool {
        for chtmp in vct {
            if chtmp == ch {
                return true;
            }
        }
        false
    }

    let mut new_word = word.clone();
    for (idx, ch) in word.char_indices() {
        if in_set(&ch, &univ_alpha) {
            // 元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）
            println!("开始移动  2 ");
            new_word.push_str("-hay");
            return new_word
        }

        if in_set(&ch, &con_alpha) {
            // 第一个辅音字母被移动到单词的结尾并增加 “ay”
            println!("开始移动  1 ");
            new_word.remove(idx);
            new_word.push(ch);
            new_word.push_str("-ay");
            return new_word;
        }

        new_word.push(ch);
    }

    new_word
}


//fn add_employee_to_dep(emp: &String) { //这里应该用 &String 还是 String??
fn add_emp_to_depart(cmd : String,  dpt: &mut HashMap<String, Vec<String>> ) -> bool{
    println!("add_emp_to_depart");

    assert!(cmd.starts_with("Add"));

    if (cmd.contains("Add") | cmd.contains("add")) &&
         (cmd.contains(" to ") | cmd.contains(" To ")){
        println!("is ok");

        let mut iter = cmd.split_whitespace();
        assert_eq!(Some("Add"), iter.next());
        let name = iter.next().unwrap();
        assert_eq!(Some("to"), iter.next());
        let depart = iter.next().unwrap();
        
        println!("name:{}, depart:{}", name, depart);


        let v = dpt.entry(depart.to_string()).or_insert(Vec::new());
        if !v.contains(&name.to_string()) {
            v.push(name.to_string());
        }

        println!("{:?}", dpt);

    }else {
        panic!("invalid cmd");
    }

    true
}


fn list_department_employee(dptname: &String,  dpt: &HashMap<String, Vec<String>>) {

    if dpt.contains_key(dptname) {
        let opt = dpt.get(dptname);
        if opt.is_some() {
            for emp in opt.unwrap() {
                println!("#{:?}#", emp); //TODO: 怎么是 vector??
            }
        }
    }

}




fn main() {

    let vct = vec!{ 2, 1, 5, 4, 9, 6, 9, 3 };
    let n_middle =  get_middle_number(&vct);
    println!("middle number : {}", n_middle);

    let n_mode = get_mode_number(&vct);
    println!("{}", n_mode);


    let avr = get_average_num(&vct);
    println!("avr:{}", avr);


    let word = String::from("first");
    let new_word = pig_latin(&word);
    println!("new_word:{}", new_word);
    
    let apple = String::from("apple");
    let new_apple = pig_latin(&apple);
    println!("new_apple: {}", new_apple);


    // let cmd = String::from("Add Sally to Engineering");
    // let cmd = String::from(" Sally to Engineering");
    let mut dpt : HashMap<String, Vec<String>> = HashMap::new();
    add_emp_to_depart("Add Sally to Engineering".to_string(), &mut dpt);
    add_emp_to_depart("Add Yqq to Engineering".to_string(), &mut dpt);
    add_emp_to_depart("Add Jack to Research".to_string(), &mut dpt);
    add_emp_to_depart("Add Tom to Worker".to_string(), &mut dpt);
    add_emp_to_depart("Add Alic to Seller".to_string(), &mut dpt);
    add_emp_to_depart("Add Atom to Seller".to_string(), &mut dpt);
    add_emp_to_depart("Add Bob to Seller".to_string(), &mut dpt);


    list_department_employee( &String::from("Seller"), &dpt);
}