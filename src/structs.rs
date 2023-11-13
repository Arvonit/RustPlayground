// Normal C-style struct
#[derive(Debug)] // This is to be able to pretty-print
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit struct
#[derive(Debug)]
struct AlwaysEqual;

pub fn run() {
    let user = User {
        email: String::from("me@example.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };
    let user = build_user(String::from("me@example.com"), String::from("username"));
    println!("{:?}", user);

    // Since we are using struct update syntax and thus borrowing values from `user`, `user` is no
    // longer valid
    let another_user = User {
        email: String::from("another@example.com"),
        ..user
    };
    println!("{:?}", another_user);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", origin);
    println!("{:?}", black);

    let subject = AlwaysEqual;
    println!("{:?}", subject);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
