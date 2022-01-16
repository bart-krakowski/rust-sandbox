use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    // Example 1
    let a = 5;
    let b = &a;

    assert_eq!(5, a);
    assert_eq!(5, *b);

    // Example 2
    let c = 5;
    let d = Box::new(c);

    assert_eq!(5, c);
    assert_eq!(5, *d);

    // Example 3
    let e = 5;
    let f = MyBox::new(e);

    assert_eq!(5, e);
    assert_eq!(5, *f); c

    // Example 4
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
