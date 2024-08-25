fn main() {
    
    // if we want to update a variable, we can use the `mut` keyword to make it mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // const, we can't use mut, and we must specify the type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // shadowing, we can declare a new variable with the same name
    // Type Change: Shadowing allows you to change the type of the variable.
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let y = "hello";
    println!("The value of y is: {y}");


    // FLOATING POINT TYPES
    let fx = 2.0; // f64

    let fy: f32 = 3.0; // f32

    println!("The value of fx is: {fx}");
    println!("The value of fy is: {fy}");

    // BOOLEAN TYPE
    let bt = true;

    let bf: bool = false; // with explicit type annotation

    println!("The value of bt is: {bt}");
    println!("The value of bf is: {bf}");

    // CHARACTER TYPE

    let cc = 'z';
    let cz: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("The value of cc is: {cc}");
    println!("The value of cz is: {cz}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // COMPOUND TYPES
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    
    // TUPLES
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // _ if we don't use it
    let (_ttx, tty, _ttz) = tup;

    println!("The value of tty is: {tty}");



    let tt2x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tt2x.0;

    let six_point_four = tt2x.1;

    let one = tt2x.2;
    
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");


    let aa: [i32; 5] = [1, 2, 3, 4, 5];
    let el1 = aa[0];
    println!("The value of aa[0] is: {el1}");

    let aai = [3; 5]; // let a = [3, 3, 3, 3, 3];
    let el2 = aai[0];
    println!("The value of aai[0] is: {el2}");

    // ############
    // FUNCTIONS
    // ############
    another_function(5);

    print_labeled_measurement(5, 'm');
}

// im rust we use snake case for function names
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}