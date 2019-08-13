


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}



enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter( UsState ),
}


/*
fn coin_matches(coin : Coin) -> u32 {

    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter( state ) =>{
            println!("State quarter from {:?}", state);
            25
        } ,
    }
}
*/

fn coin_matches(coin : Coin) -> u32 {

    let mut count = 0;
    match coin {
        /*Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,*/

        Coin::Quarter( state ) =>{
            println!("State quarter from {:?}", state);
            count = 25;
        } ,
        //_ => count + 1
        _ =>{
            println!("+=1 " );
            count += 1;
        } 
    }
    

    return count;
}



fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None =>  None,
    }
}



fn main() {
    //println!("Hello, world!");

    let value  = coin_matches( Coin::Dime );
    let coin2 = coin_matches(Coin::Quarter(UsState::Alabama));
    // let coin2 = coin_matches(Coin::Quarter); //error
    println!("value:{}", value);
    println!("coin2:{}", coin2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);



    let coin3 = Coin::Penny;
    let mut count = 0;
    
    if let Coin::Quarter(state) = coin3 {
        println!("State quarter from {:?}!", state );
    }else{
        count += 1
    }

    println!("{}", count);
}

