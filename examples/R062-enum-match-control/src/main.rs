#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Penny);
    other_example(29);
}

fn value_in_cents(coin: Coin) -> u8 {
    // with if we validate a boolean, with match we validate any value
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }



   
}

fn other_example(dice_roll: u8) -> u8 {
    match dice_roll {
        3 =>{ 
            println!("You rolled a 3!");
            3
        },
        7 => {
            println!("You rolled a 7!");
            7
        }
        // we can use _ if we don't want the value
        other => {
            println!("You rolled a {}!", other);
            other
        }
    }
}