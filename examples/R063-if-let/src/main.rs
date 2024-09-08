enum Option<T> {
    None,
    Some(T),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    
    let config_max = Option::Some(3u8);
    if let Option::Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let coin1 = Coin::Quarter(UsState::Alaska);
    //let coin2 = Coin::Penny;

    if let Coin::Quarter(state) = coin1 {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
        println!("Not a quarter, count is {count}");
    }
}
