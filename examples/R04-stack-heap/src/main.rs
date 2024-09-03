fn main() {

    let s1 = String::from("hello");
    let s2 = s1; // same memory address

    // after the line let s2 = s1;, Rust considers s1 as no longer valid.
    // println!("{}, world!", s1);
    // no shallow copy, it’s known as a move. We would say that s1 was moved into s2
    // Rust will never automatically create “deep” copies of your data



    // To create a deep copy, both the stack and heap data
    let sc1 = String::from("hello");
    let sc2 = s1.clone();

    println!("sc1 = {sc1}, sc2 = {sc2}");
}
