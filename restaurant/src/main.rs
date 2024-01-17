#!/usr/bin/rustc
mod rectangle;
use crate::rectangle::{area, division,state_name, check_sum};
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
}
