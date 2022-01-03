use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new();

    v.push(1);

    let mut v2 = vec![1, 2, 3, 4, 5];
    
    let third = &v[2];
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }


    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

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

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        SpreadsheetCell::Float(f) => println!("{}", f),
        SpreadsheetCell::Text(s) => println!("{}", s),

        _ => println!("unknown"),
    }


    // Strings
    let s1 = String::new();
    let s2 = "String";
    let s3 = s2.to_string();
    let s4 = String::from("String");
    
    let mut s5 = String::from("foo");
    s5.push_str("bar");
    s5.push('x');
    
    let s6 = String::from("hello");
    let s7 = String::from("world");

    // let s8 = s6 + &s7;
    // println!("{}", s6);

    let s9 = format!("{} {}", s6, s7);

    let hello = String::from("hello");
    for b in "hello".bytes() {
        println!("{}", b);
    }

    for g in "hello".graphemes(true) {
        println!("{}", g);
    }


    // Hashmaps
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    let mut scores2: HashMap<String, i32> = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Yellow"), 50);

    scores2.entry(String::from("Yellow")).or_insert(30);
    scores2.entry(String::from("Yellow")).or_insert(50);


    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
