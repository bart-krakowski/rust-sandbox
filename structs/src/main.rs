#![allow(dead_code)]
#![allow(unused_variables)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("bogdan@gmail.com"),
        username: String::from("bogdan"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("bogdan2");

    let user2 = build_user(String::from("kyle@mail.com"), String::from("Kyle"));

    let user3 = User {
        email: String::from("james@mail.com"),
        username: String::from("james"),
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle { width: 30, height: 50 };
    rect.area();
    println!("rect: {:#?}", rect);

    let rect1 = Rectangle { width: 20, height: 40 };
    let rect2 = Rectangle { width: 40, height: 50 };
    println!("rect can hold rect1: {}", rect1.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::square(25);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
