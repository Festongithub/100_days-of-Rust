#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String, 
    age: u8,
}

struct Unit;
struct Pair(i32, f32);

struct Point {
    x:f32,
    y: f32,
}
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}


fn main() {
    println!("Hello, world!");

    let name = String::from("Peter Griffins");
    let age = 78;
    let peter = Person {name, age};

    let p = Point { x: 89.79, y:98.43 };
    println!("x is : {}", p.x);

    let r = Rectangle {
        top_left: p
        bottom_right: p
    }

    println!(" total area is : {:?}", r.top_left);

    println!("my name is : {:?} hahaha funny right!", peter.name);
}
