#!/usr/bin/rust

fn another_function() {
    println!("Welcome to Rust");
}

fn add(x: i32) -> i32 {
    return x + x;
}

fn sub(x: u32, y: u32) -> u32 {
    return x - y;
}

fn user_details(number:i32, name: char) -> (i32, char) {
    println!("{}: {}", number, name);
    (number, name)
}

// control flow

fn main() {
    another_function();
    println!("{}", add(78));
    println!("{}", sub(100, 10));
    user_details(3, 'L');
    loop {
        println!("again");
    }

}
