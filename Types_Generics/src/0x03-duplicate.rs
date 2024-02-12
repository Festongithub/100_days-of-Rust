#!/usr/bin/rustc

fn check_i32<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut item = &list[0];

    for i in list {
        if i > item {
            item = i;
        }
    }
    item
}

use std::ops::Add;

fn add<T: Add<Output = T>>(a:T, b: T) ->T {a+b}

use std::ops::Mul;
fn mul<T: Mul<Output = T>>(a:T, b:T) ->T{a * b}

#[derive(Debug)]
struct Point<T>
{
    x: T,
    y: T,
}
#[derive(Debug)]
struct Student<T, U>
{
    age: T,
    number: T,
    name: U,
    email: U,
}


fn main()
{
    println!("sum is: {}", add(78, 89));
    println!("Product is: {}", mul(89, 90));

    let num_list = vec![90, 78, 54, 45, 23];

    println!("Elements are {}:",check_i32(&num_list));


    let integer = Point {x: 5, y: 10};
    let float = Point { x: 4.5, y: 4.8};

    let s_1 = Student {age: 67, number:37, name:"Festus".to_string(), email:"fkemboik@gmail.com".to_string()};
    println!{"{:?}", s_1};

    println!{"{:?}", float};

}
