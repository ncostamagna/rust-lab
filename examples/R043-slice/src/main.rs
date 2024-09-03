fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!




    let s2 = String::from("hello world");

    // reference to a portion of a string
    let hello = &s2[0..5]; // [starting_index..ending_index]
    let world = &s2[6..11];
    println!("{hello} {world}");

    {
        let s = String::from("hello");
        let slice = &s[..2]; // same that &s[0..2];
        println!("{slice}");
    }

    {
        let s = String::from("hello");

        let len = s.len();
        let slice = &s[3..]; // same that &s[3..len]
        println!("{slice}");
    }

    {
        let s = String::from("hello");

        let len = s.len();
        let slice = &s[..]; // same that  &s[0..len];
        println!("{slice}");
    }




    let mut s4 = String::from("hello world");
    let word = first_word_good(&s4);

    println!("the first word is: {word}");

    s4.clear(); // error! if I issue the follow println! affter

    // println!("the first word is: {word}");






    let my_string = String::from("hello world");

    // `first_word_good` works on slices of `String`s, whether partial or whole
    let word = first_word_good(&my_string[0..6]);
    let word = first_word_good(&my_string[..]);
    // `first_word_good` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_good(&my_string);

    let my_string_literal = "hello world";

    // `first_word_good` works on slices of string literals, whether partial or whole
    let word = first_word_good(&my_string_literal[0..6]);
    let word = first_word_good(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_good(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to array of bytes

    // iterate over the array of bytes with iter method
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b' ' is byte literal syntax
            return i;
        }
    }

    s.len() // PROBLEM: it’s a separate value from the String, there’s no guarantee that it will still be valid in the future
}

// with str we can use both (string and str), with string we can only use string
fn first_word_good(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}