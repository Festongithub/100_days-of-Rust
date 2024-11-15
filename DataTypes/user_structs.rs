#!/usr/bin/env rustc

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl User {
    fn name(&self) -> &String {
        return &self.name;
    }
    fn age(&self) -> u32 {
        return self.age;
    }
}


fn main() {
    let m = 90;
    let rect = Rectangle {
        width: dbg!(89 * m),
        height: dbg!(90 * m),
    };

    let student = User {
        name: dbg!(String::from("Hendry")),
        age: 23,
    };

    dbg!(&student);
    dbg!(&rect);

    println!(
        "The area of the rectangle is {} square",
        rect.area()
    );
    println!(
        "Name is {}", 
        student.name()
    );
    println!(
        "Age is {}", 
        student.age()
    );
}