struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &String) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 6;
    let y = MyBox::new(x);

    assert_eq!(3+3 , x);
    assert_eq!(6+0, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
