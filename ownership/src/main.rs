#![allow(unused_variables)]

fn main() {
    // Ownership rules:
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let x = 5;
    let y = x; // Copy
    
    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // Error: s is invalid after this line

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s3 = String::from("hello");
    let len = calculate_length(&s3);
    println!("The length of '{}' is {}.", s3, len);

    let mut s4 = String::from("hello");
    change(&mut s4);

    let mut s5 = String::from("hello world");
    let hello = &s5[..5];
    let world = &s5[..];
    let word = first_word(&s5);
    s5.clear();

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}