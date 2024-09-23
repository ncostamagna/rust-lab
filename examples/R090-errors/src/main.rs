enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;
use std::io::ErrorKind;


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

    // continue in https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors

}
