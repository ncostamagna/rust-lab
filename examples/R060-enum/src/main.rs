use std::ops::Add;

// without values
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrEnm {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


enum Option<T> {
    None,
    Some(T),
}


impl Add for Option<i8> {
    type Output = Option<i8>;

    fn add(self, other: Option<i8>) -> Option<i8> {
        match (self, other) {
            (Option::Some(a), Option::Some(b)) => Option::Some(a + b),
            _ => Option::None,
        }
    }
}


fn main() {
    

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrEnm::V4(127, 0, 0, 1);

    let loopback = IpAddrEnm::V6(String::from("::1"));

    println!("home is {:?}", home);
    println!("home is {:?}", loopback);


    //let x: i8 = 5;
    //let x: Option<i8> = Some(2);
    //let y: Option<i8> = Some(5);

    let x = Option::Some(5);
    let y = Option::Some(10);

    let sum = x + y;

    match sum {
        Option::Some(value) => println!("Sum: {}", value),
        Option::None => println!("Sum: None"),
    }

}

// other example in rust doc
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}