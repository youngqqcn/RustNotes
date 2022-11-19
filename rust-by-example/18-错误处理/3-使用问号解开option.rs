// Author: yqq
// Date: 2022-11-19 21:01:27
// Description:

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}



fn main() {

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber{
                area_code: Some(61),
                number: 3333333333,
            }),
        }),
    };

    println!("{:?}", p.work_phone_area_code());

    let p = Person {
        job: Some(Job {
            phone_number: None,
        }),
    };

    println!("{:?}", p.work_phone_area_code());
}