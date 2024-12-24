use std::string;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}",user2.email);
    println!("{}", user1.email);

    let black = Color(0, 1, 0);
    let origin = Color(0, 0, 0);

    let user3 = build_user(String::from("build@example.com"), String::from("pty"));

    println!("{}",user3.email);
    println!("{}",black.1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("The area of rectangle is {} square pixels.", area(&rect1));
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}