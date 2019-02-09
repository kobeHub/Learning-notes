// define the Coin enumeration
#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),    // every quater of different states has different back
}

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
}

// get the value of the given Coin
pub fn value_in_coin(coin: &Coin) -> u32 {
    match coin{
        Coin::Penny => {
            println!("Lucky coin");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}", state);
            25
        },
    }
}

// deal with the Quarter case
pub fn quarter(coin: &Coin) {
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        println!("not quarter coin");
    }
}
