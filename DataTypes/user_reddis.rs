#!/usr/bin/rustc

fn division(a: u32, b: u32) -> u32 {
    if b == 0 {
        return 0;
    } else {
        return a / b;
    }
}


fn get_names(name: String, age: i32) {
    if age < 18 {
        println!("Hey {name}!Too young to be here")
    } else {
        println!("Student is: {name} aged :{age} years");
    }
}

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    division(90, 10);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!{"The Value is: {}", a[index]}
        index += 1;
    }

    for i in a {
        println!("The value is:{i}");
    }
    println!("You made it");

    let s1 = String::from("Baddy");
    let num = 16;
    get_names(s1, num);

    let p = 670;
    let q = 10000;

    //revenue(p, q);

    //println!("s1 = {s1}, s2 ={s2}")
}
