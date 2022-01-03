// #[allow(overflowing_literals)]
#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let x: u32 = 5;
    println!("The values of x is: {}", x);
    let x = 6;
    println!("The values of x is: {}", x);
    let x = "Test";
    println!("The values of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;

    let a: u8 = 255;
    println!("The values of a is: {}", a);

    // Tuples
    let tup = ("String", 100_000);
    let (string, number) = tup;
    let b = tup.1;

    my_fn();
}


fn my_fn() -> char {
 let a = 'a';
 a
}


// Control Flow
fn control_flow() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
}

// Loops
fn loops() {
    let mut counter = 0;
    let mut number = 0;

    // Basic loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    // While loop
    while number != 0 {
        number -= 1;
    }

    // For loop
    let a = [10, 20, 30, 40, 50, 60];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("The value is: {}", number);
    }
}