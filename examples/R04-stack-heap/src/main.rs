fn main() {

    let s1 = String::from("hello");
    let s2 = s1; // same memory address

    // after the line let s2 = s1;, Rust considers s1 as no longer valid.
    // println!("{}, world!", s1);
    // no shallow copy, it’s known as a move. We would say that s1 was moved into s2
    // Rust will never automatically create “deep” copies of your data



    // To create a deep copy, both the stack and heap data
    let sc1 = String::from("hello");
    let sc2 = sc1.clone();

    println!("sc1 = {sc1}, sc2 = {sc2}");



    // in case of integers, it's ok
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    /* 
    types such as integers that have a known size at compile time are stored entirely on the stack
    
    There are some of the types that implement Copy:

    All the integer types, such as u32.
    The Boolean type, bool, with values true and false.
    All the floating-point types, such as f64.
    The character type, char.
    Tuples, if they only contain types that also implement Copy
    */

    /*
    The mechanics of passing a value to a function are similar to those when assigning a value to a variable
     */
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward



    /* Returning values can also transfer ownership. */

    let sf1 = gives_ownership();         // gives_ownership moves its return
                                        // value into sf1

    let sf2 = String::from("hello");     // sf2 comes into scope

    let sf3 = takes_and_gives_back(sf2);  // sf2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into sf3

  // {when the scope end} Here, sf3 goes out of scope and is dropped. sf2 was moved, so nothing
  // happens. sf1 goes out of scope and is dropped.                   
}



fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.






fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

    a_string  // a_string is returned and moves out to the calling function
}