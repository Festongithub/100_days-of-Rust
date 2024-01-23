#!/usr/bin/rustc
pub fn area(w: u32, h:u32) -> u32 { w * h}

pub fn division(i : u32, j: u32) -> u32 { i % j}

pub fn state_name(name: String) {
    println!("Another module {}", name);
}

pub fn check_sum(x: u32){
    if x > 0 {
        println!("{} is positive", x);
    }
    if x <= 0 {
        println!("{} is negative", x);
    }
    if x % 2 == 0 {
        println!("{} is divisible by 2", x);
    }

    if x % 2 != 0  {
        println!("{} is not divisible by 2",x);
    }
}

pub fn call_function() {}

