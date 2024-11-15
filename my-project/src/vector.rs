#!/usr/bin/env rustc

fn  main() {
let mut numbers = Vec::new();

numbers.push(56);
numbers.push(57);
numbers.push(58);
numbers.push(59);
numbers.push(60);

// accessing elements of vectors
// indexing && using get method

let second = &numbers[2];
println!("The second element is {second}");

let s: Option<&i32> = numbers.get(3);
    match s {
        Some(s) => println!("The fourth element is {s}"),
        None => println!("No element found!")
    }

for i in &numbers {
    let sum = i ** i;
    println!("{sum}")
}

let cities:Vec<String> = vec!["Nakuru".to_string(), "Momabasa".to_string()];

for city in &cities {
    println!("{city}")
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(78),
    SpreadsheetCell::Float(12.0909),
    SpreadsheetCell::Text(String::from("Blue"))
];

for item in &row {
    println!("{}", item)
}
}
