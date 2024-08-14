// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#using-a-crate-to-get-more-functionality

use std::io;

// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods
use rand::Rng; 


use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // range, if we want to create a range, use start..=end (= 100 too)

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // we can update the valiue in this variable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
