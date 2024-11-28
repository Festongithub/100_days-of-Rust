enum List{
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(89);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


    let x = 7;
    let y = Box::new(x);

    assert_eq!(7, x);
    assert_eq!(7, *y);

    println!("The Value of y is {y}");

}
