#!/usr/bin/rustc

// getting multiple values
#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}



fn main() {


    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(78.98),
        SpreadSheetCell::Text(String::from("Blue")),
    ];

    for j in &row {
        println!("{:#?}", j)
    }

    let mut numbers = Vec::new();

    numbers.push(78);
    numbers.push(89);
    numbers.push(88);
    numbers.push(90);

    let third: &i32 = &numbers[2];
    println!("The Third element is {third}");

    let fourth: Option<&i32> = numbers.get(3);
    match fourth {
        Some(fourth) => println!("Fourth element is {fourth}"),
        None => println!("No fourth element."),
    }

    for i in &mut numbers {
        *i += 50;
        println!("{i}");
    }

}

