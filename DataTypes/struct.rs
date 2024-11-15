#!/usr/bin/rustc

const THREE_PRICE: i32 = 9000;
const TEN_POINT: f32 = 90.98;

fn add(a:i32, b:i32) -> i32 {
    return a + b;
}

fn division(a:f32, b:f32) -> f32 {
    if b == 0.0 {
        return 0.0;
    } else {
        return a / b;
    }
}

fn visitor_checker(phone_number: u32, label: char){
    println!("Visitors number is {phone_number} and label: {label}")
}

fn number_increment(i: i32) -> i32 {
    let j = 12 + i;
    return j;   
}

fn call_message(star: char) -> char {
    loop {
        printlin!{"message is: {}", star};
    }
}


fn main() {
    let mut x = 90;
    println!("The Value of x is: {x}");
    x = 78;
    println!("new value of {x}");

    let p = x * THREE_PRICE;
    println!("Value of {p}");

    //Floats operations can only work on float numbers
    // character types

    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");

    let tup = (800, 7.8, 1);
    let (x, y, z) = tup;
    println!("Value of: {y}");

    let cities = ("Nairobi", "Paris", "Pineas");
    let (_kenya, _france, _mexico) = cities;

    println!("The city of Kenya: {_kenya}");

    // test function
    let sum = add(34, 56);
    println!("Total sum is: {sum}");

    let q = division(72.0, 16.0);
    println!("quotiennt: {q}");

    visitor_checker(67989989, 'H');
    call_message('c');
}