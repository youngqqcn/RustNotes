// Author: yqq
// Date: 2022-11-10 22:43:12
// Description:

// 即为不同种类的数据分别定义新的类型

struct Years(i64);
struct Days(i64);


impl Years {
    pub fn to_days(&self) -> Days{
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 10
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();

    println!("{}", old_enough(&age));
    println!("{}", old_enough(&age_days.to_years()));
}