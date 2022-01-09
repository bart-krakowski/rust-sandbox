// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.

use std::fmt::Display;

struct _ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> _ImportantExcerpt<'a> {
    fn _return_part(&self, annoucement: &str) -> &str {
        println!("Attension please: {}", annoucement);
        self.part
    }
}

fn main() {
    // Example 1
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    // Example 2
    // let x = 5;
    // let r = &x;

    // println!("r: {}", r);

    // Example 3
    // let string1 = String::from("abcd");
    // let string2 = String::from("xyz");

    // let result = longest(string1.as_str(), string2.as_str());
    // println!("The longest string is {}", result);

    // Example 4
    // let string1 = String::from("abcd");
    // {
    //     let string2 = String::from("xyz");

    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result);
    // }

    // Example 5
    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");

    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // Example 6
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //     let i = ImportantExcerpt {
    //         part: first_sentence,
    //     };

    // Example 7
    // let s: &'static str = "I have a static lifetime.";
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn _longest<'a>(_x: &str, _y: &str) -> String {
    // if x.len() > y.len() {
    //     x
    // } else {
    //     y
    // }

    let result = String::from("really long string");
    result
}

fn _first_word(x: &str) -> &str {
    let bytes = x.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &x[0..i];
        }
    }

    &x[..]
}

fn _longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display,
{
    println!("Annoucement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
