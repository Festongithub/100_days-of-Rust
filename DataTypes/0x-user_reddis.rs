#!/usr/bin/env rustc

fn length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn main() {
    let mut message = String::from("Hello world");
    let len = length(&message);
    change(&mut message);

    let m = String::from("Build it from the ground");

    let hello = &m[0..5];
    let world = &m[..];

    println!("range of hello: {hello}");
    println!("range of world: {world}");

    println!("Lenght of the text: {len}");
}
