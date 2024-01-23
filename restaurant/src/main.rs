#!/usr/bin/rustc
use std::collections::HashMap;
mod rectangle;
pub use crate::rectangle::*;
fn main()
{
    let i = 90;
    let j = 91;
    println!("Area is {}", area(i, j));
    println!("Quotient is {}", division(i, j));
    println!("Modules");
    let name = String::from("Good life");
    state_name(name);

    check_sum(i);
    check_sum(j);
    call_function();

    let mut map = HashMap::new();
    map.insert(1, 3);
}
