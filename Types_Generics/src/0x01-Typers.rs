#!/usr/bin/rustc

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // The function largest is generic over some type T .This function has one parameter ==> list 
    // return ==> value of the same type T

    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> { 
    fn x(&self) -> &T {
        &self.x 
    }
}

// Alternatively we can use struct with different types

#[derive(Debug)]

struct Student<T, U> {
    name: T,
    age: U,
}

impl <T,U> Student<T, U>{
    fn list(&self, &self) ->&T, &U {
        &self.list , &self.list
    }
}
fn main()
{
    let numbers = vec![567, 345, 7890, 12000];
    println!("{} is the largest", largest(&numbers));

    let letters = vec!['z', 't', 'y', 'i'];
    println!("{} {} is the largest", largest(&letters), largest(&numbers));

    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());


    let s_1 = Student {name: "Felix", age: 13};
    let s_2 = Student {name: "Macron", age: 14};


    println!("s_1 = {:?}", s_1.list());
    println!("s_2 = {:?}", s_2.list());
}
