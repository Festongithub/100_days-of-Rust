static WORD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

use std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other:Point) -> Point {
        Point {
            x: self.x + other.x,
            y : self.y + other.y,
        }
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn word() {
    println!("word is: {WORD}");
}

fn main() {
    word();
    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }

    assert_eq!(
        Point { x: 1, y: 0} + Point { x: 2, y: 3},
        Point {x : 3, y: 3}
        );
}
