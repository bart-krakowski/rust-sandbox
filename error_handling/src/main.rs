#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::{File, read_to_string}, io::{ErrorKind, Error}};

fn main() {
    let file = File::open("hello.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });



    // let file = File::open("hello.txt").unwrap();

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    let file = File::open("hello.txt").expect("Failed to open the file: {:?}");


    // Error propagation

    fn read_username_from_file() -> Result<String, Error> {
        let s = String::new();
        // File::open("hello.txt")?.read_to_string(&mut s)?;

         read_to_string("hello.txt")
    }
}
