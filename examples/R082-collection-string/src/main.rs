fn main() {
    let s1 = String::from("hello"); // 4 bytes
    // error: let h = s1[0];

    // let hello = String::from("Здравствуйте"); 24 bytes - each Unicode scalar value in that string takes 2 bytes of storage
    // Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value

    let hello = "Здравствуйте"; // it isn't 3, it's another character
    let s = &hello[0..4]; // &hello[0..1] -> panic in the same way that index[0]
    println!("{}", s);

    let hello2 = "abcdefgh";
    let s2 = &hello2[0..4];
    println!("{}", s2);


    // it's not an index it's a char

    // methods for iterating over strings
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Здnñ3".bytes() {
        println!("{b}");
    }
}
