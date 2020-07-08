
pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {

    pub fn new()-> AverageCollection {
        AverageCollection{
            list:vec![], 
            average:0f64
        }
    }


    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average(); //更新平均值
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


fn main() {

    // let mut avr = AverageCollection{list:vec![], average:0f64};
    let mut avr = AverageCollection::new();
    for i in 0..100 {
        avr.add(i);
    }
    println!("average:{}", avr.average());
    println!("average:{}", avr.average());


}