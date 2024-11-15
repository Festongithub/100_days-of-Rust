use std::ops::Mul;

struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T,U, V> Rectangle<T, U> {
    where
        T : Mul<U, Output=V>,
    {
        fn area(&self) -> V {
            self.width * self.height
        }
    }
}

fn main() {
    let r = Rectangle {width: 80, height: 90};

    println!("Th area is : {}", r.area());
}
