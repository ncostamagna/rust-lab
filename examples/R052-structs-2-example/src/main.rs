
// with this value we can print the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect2)
    );

    // println!("rect1 is {}", rect1);   error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    println!("rect1 is {rect2:#?}");

    //we can use for showing the struct
    dbg!(rect2);

}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}