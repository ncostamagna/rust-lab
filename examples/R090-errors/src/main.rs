/*enum Result<T, E> {
    Ok(T),
    Err(E),
}*/

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let v = vec![1, 2, 3];

    // unrecoverable error
    // v[99];


    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    /* alternative to using match with Result<T, E> 
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    */

    /*
    Using match works well enough, but it can be a bit verbose and doesn’t always 
    communicate intent well. The Result<T, E> type has many helper methods defined 
    on it to do various, more specific tasks. The unwrap method is a shortcut method 
    implemented just like the match expression. 
    If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
    If the Result is the Err variant, unwrap will call the panic! macro for us. 
     */

     let greeting_file = File::open("hello.txt").unwrap();
      println!("{:?}", greeting_file);

      /*
      Similarly, the expect method lets us also choose the panic! error message. 
      Using expect instead of unwrap and providing good error messages can convey 
      your intent and make tracking down the source of a panic easier. 
       */
      let greeting_file = File::open("hello.txt")
      .expect("hello.txt should be included in this project");
        println!("{:?}", greeting_file);
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    // The ? operator is used to simplify error handling. 
    // It automatically returns an error if the operation fails, 
    // otherwise it unwraps the Ok value.
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator