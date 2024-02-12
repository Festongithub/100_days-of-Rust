#!/usr/bin/rustc

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2> (
        self,
        other: Point<X2, Y2>,
    ) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
#[derive(Debug)]
enum OptionI32 {
    Some(i32),
    None,
}
#[derive(Debug)]
enum Optionf64 {
    Some(f64),
    None,
}
fn main()
{
    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "Hello" , y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let int = OptionI32::Some(5);
    let float = Optionf64::Some(5.0);

    println!("{:?}", int);
    println!("{:?}", float);
}
