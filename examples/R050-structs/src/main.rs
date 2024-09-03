
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{0}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("{0}", user1.email);


    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    }; // we can no longer use user1 as a whole after creating user2 (in case of username)

    println!("{0}", user1.active);

    // Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}