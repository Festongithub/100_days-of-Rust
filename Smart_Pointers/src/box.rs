use std::ops::Deref;

struct MyBox<T>(T);


impl<T> Deref for  MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}


struct Pointer {
    data: String,
}

impl Drop  for Pointer {
    fn drop(&mut self) {
        println!("Dropping Pointer with data `{}`", self.data)
    }
}
fn main() {
    let i = 90;
    let y = MyBox::new(i);

    println!("{:?}", assert_eq!(90, i));
    println!("{:?}", assert_eq!(90, *y));

    let m = MyBox::new(String::from("Rust is good"));
    hello(&m);

    let p = Pointer {
        data: String::from("My Stuff"),
    };
    drop(p);
    
    let d = Pointer {
        data: String::from("Other stuff"),
    };

    println!("Pointer created!")
}
