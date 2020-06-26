

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {

    println!("a bady dog is called a {}", Dog::baby_name());

    //  cannot infer type
    // println!("a bady dog is called a {}", Animal::baby_name());


    println!("a bady dog is called a {}", <Dog as Animal>::baby_name());

}