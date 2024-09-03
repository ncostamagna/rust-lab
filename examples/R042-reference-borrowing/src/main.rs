fn main() {
    let s1 = String::from("hello");

    // reference
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");


    // if we want to modify the value of a reference, we need to use the `mut` keyword
    //let s = String::from("hello");
    let mut s = String::from("hello");

    change(&mut s);

    /*  Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    */

    // we can use scope 
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;



    // when we use &mut s, we don't use r1 and r2 anymore
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    // Dangling References
    let reference_to_nothing = no_dangle();


}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    // here, s is reference borrowing
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

  fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!*/


  fn no_dangle() -> String {
    let s = String::from("hello");

    s
}