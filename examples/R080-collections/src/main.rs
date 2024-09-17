fn main() {

    // To create a new empty vector, we call the Vec::new functio
    // Vectors are implemented using generics
    let v: Vec<i32> = Vec::new();

    println!("{:?}", v);

    // Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);


    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("{:?}", v3);


    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


    // let does_not_exist = &v[100]; error
    // println!("The 100th element is {does_not_exist}");
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(does_not_exist) => println!("The third element is {does_not_exist}"),
        None => println!("There is no third element."),
    }


    /*
        Error:

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");


    This error is due to the way vectors work: because vectors put the values next to 
    each other in memory, adding a new element onto the end of the vector might require 
    allocating new memory and copying the old elements to the new space, if there isn’t 
    enough room to put all the elements next to each other where the vector is currently stored.
    In that case, the reference to the first element would be pointing to deallocated memory. 
    The borrowing rules prevent programs from ending up in that situation

     */




     let v = vec![100, 32, 57];
     for i in &v {
         println!("{i}");
     }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);


    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

}
