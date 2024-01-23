#!/usr/bin/rustc

fn main()
{
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(10.23),
    SpreadsheetCell::Text(String::from("blue")),
];

let points = vec![1, 3, 6, 12, 36, 108, 324];
for m in &points {
    println!("Product is {}", m * m);
}

let mut mess = String::new();
mess.push_str("Hello my fans");
println!("{}", mess);

println!("{}", mess);

let data = "Initials";
let s = data.to_string();
println!("s is {}", s);
println!("message is {}", data);


let mut test = String::new();
test.push_str("Hello world");
println!("{}", test);

// when s is created it contains an empty string, we can take it like a container with nothing, or
// an empty set, with this we can push --> some new strings into the string created .
// push() method takes a single character as a parameter

test.push('R');
test.push('u');
test.push('s');
test.push('t');
println!("{}", test);


//Concatenation with the + Operator or the format! Macro'

let sum = mess + &test;
println!("{}", sum);


let mut music_artist = String::new();
music_artist.push_str("Khaligraph jones");
println!("name of Artist is {}",music_artist);


let s1 = String::from("Love is lost in");
let s2 = String::from("the Perfume");
let s3 = String::from("You are the sugar");

let s = s1+ "_" + &s2 + "_" + &s3;
println!("The song lines is {}", s);

// Alternativel we can use thr format! Macro

let s4 = String::from("Walk into the storm");
let s5 = String::from("It aint me");

let f = format!("{s4} - {s5}");
println!("{}", f);


let mut lang = String::new();
lang.push_str("Здравствуйте");
println!("{}", lang);

let index = &lang[0..2];
println!("position is {}", index);

for c in lang.chars(){
    println!("{c}");
}

for i in s.chars()
{
    println!("{i}")
}

for b in s.bytes()
{
    println!("{b}");
}
}
