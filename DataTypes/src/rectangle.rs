#!/usr/bin/env rustc

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct Vendor {
    name: String,
    age: u32,
    email: String,
    Price: u32,
    product: String,
    Quantity: u32,
}

fn revenue(vendo1: &Vendor) -> u32 {
    vendo1.Price * vendo1.Quantity
}

fn main() {
    let rect1 = Rectangle {
        width: 567,
        height: 780,
    };
    println!("{}", area(&rect1));
    println!("rect1 is {rect1:?}");

    let vendor1 = Vendor {
        name: String::from("Emmy"),
        age: 34,
        email: String::from("Emmya@24.gmail.com"),
        Price: 345,
        product: String::from("Oranges"),
        Quantity: 678,
    };
    println!("Total Revenue: {}", revenue(&vendor1));
}
