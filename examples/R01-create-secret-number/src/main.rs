// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#using-a-crate-to-get-more-functionality

use std::io;

// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods
use rand::Rng; 

// Ordering is another enum, like Result. It has variants Less, Greater, and Equal
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // range, if we want to create a range, use start..=end (= 100 too)

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    // loop every try
    loop {
        println!("Please input your guess.");

        // we can update the valiue in this variable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing the previous. Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess
        // remove blank spaces and convert to number (parse with :u32)
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            },
        };
        
        println!("You guessed: {guess}");

    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // if is it the number, break the loop
                println!("You win!");
                break;
            }
        }
    }
}
