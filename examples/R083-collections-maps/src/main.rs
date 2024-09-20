use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score of Blue team: {}", score);
    
    scores.entry(String::from("Red")).or_insert(50); // add only if it doesn't exist
    scores.entry(String::from("Blue")).or_insert(50); 

    for (key, value) in &scores {
        println!("{key}: {value}");
    }



    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // The or_insert method returns a mutable reference (&mut V) 
        *count += 1;
    }

    println!("{map:?}");


}
