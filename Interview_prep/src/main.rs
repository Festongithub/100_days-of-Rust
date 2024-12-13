
fn add(a: u32, b:u32) -> u32 {
    return a + b;
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y:y}
    }
}

fn main() {
    println!("Hello, world!");

    let i = 900;
    let j = 456;
    println!("{} + {} = {}", i, j, add(i,j));
}
