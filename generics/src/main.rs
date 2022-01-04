#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    // Example 1
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest_number = get_largest(number_list);
    let largest_char = get_largest(char_list);

    println!("{}", largest_number);
    println!("{}", largest_char);


    // Example 2
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5, y: 10.0 };


    // Example 3
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    

    // Example 4
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p3 = Point { x: 5.0, y: 10.4 };
    let p4 = Point { x: "Hello", y: 'c' };
    let p5 = p3.mixup(p4);
}

struct Point<T, U> { x: T, y: U }

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64, f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
