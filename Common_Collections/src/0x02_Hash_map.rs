#!/usr/bin/rustc
use std::collections::HashMap;
fn main()
{
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 30);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Indigo"), 60);
    scores.insert(String::from("Break"), 80);
    for (key, value) in &scores {
    println!("{key} : {value}");
}
}
