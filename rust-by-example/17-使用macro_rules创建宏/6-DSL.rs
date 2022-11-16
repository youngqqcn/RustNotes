// Author: yqq
// Date: 2022-11-16 22:45:53
// Description: DSL


macro_rules! calculate {
    (eval666 $e: expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}



fn main() {

    calculate! {
        eval666 1 + 2
    }


    calculate! {
        eval666 (1 + 2) * (3 / 4)
    }


}