
// pub trait Messager {
//     fn send(&mut self, msg: &str);
// }


// pub struct LimitTracker<'a, T: Messager> {
//     messenger: &'a mut T, // 这里有引用, 必须声明生命周期
//     value: usize,
//     max: usize,
// }


// impl<'a, T> LimitTracker<'a, T>  where T: Messager
// {
//     pub fn new(messenger: &mut T, max: usize) -> LimitTracker<T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value:usize) {
//         self.value = value;

//         let percentage_of_max = self.value as f64 / self.max as f64;

//         if percentage_of_max > 1.0 {
//             self.messenger.send("Error: you are over you quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger.send("Urgent warning: you've used over 90% of your quota!");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger.send("Warning: you've use up over 75% of your quota!");
//         }
//     }
// }



// struct MockMessenger {
//     sent_messages: Vec<String>,
// }

// impl MockMessenger {
//     fn new() -> MockMessenger {
//         MockMessenger{
//             sent_messages: vec![]
//         }
//     }
// }

// impl Messager for MockMessenger {
//     fn send(&mut self, message: &str) {
//         self.sent_messages.push(String::from(message));
//     }
// }

    
// fn main() {

//     // let mock_msgr = MockMessenger::new();
//     // let mut limit_tracker = LimitTracker::new(&mock_msgr, 100);
//     //  `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable

//     let mut mock_msgr = MockMessenger::new();
//     let mut limit_tracker = LimitTracker::new(&mut mock_msgr, 100); 

//     limit_tracker.set_value(80);

//     println!("{}", mock_msgr.sent_messages.len());

// }



use std::cell::RefCell;
// use std::cell::RefMut;

pub trait Messager {
    fn send(&self, msg: &str);
}


pub struct LimitTracker<'a, T: Messager> {
    messenger: &'a T, // 这里有引用, 必须声明生命周期
    value: usize,
    max: usize,
}


impl<'a, T> LimitTracker<'a, T>  where T: Messager
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value:usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max > 1.0 {
            self.messenger.send("Error: you are over you quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: you've used over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: you've use up over 75% of your quota!");
        }
    }
}



struct MockMessenger {
    // sent_messages: Vec<String>,
    sent_messages: RefCell< Vec<String> >, 
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger{
            sent_messages:  RefCell::new( vec![] )
        }
    }
}

impl Messager for MockMessenger {
    fn send(&self, message: &str) {
        // self.sent_messages.borrow_mut().push(String::from(message));

        // thread 'main' panicked at 'already borrowed: BorrowMutError'
        let mut first_borrow = self.sent_messages.borrow_mut();
        first_borrow.push(String::from(message));

        let mut second_borrow = self.sent_messages.borrow_mut(); 
        second_borrow.push(String::from(message));
    }
}

    
fn main() {


    let mock_msgr = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_msgr, 100); 

    limit_tracker.set_value(80);

    println!("{}", mock_msgr.sent_messages.borrow().len());

}



