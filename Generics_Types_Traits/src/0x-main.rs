#!/usr/bin/rustc

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }
}

fn largest<T:std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}


fn main() {

    let integer = Point { x: 780, y: 8900 };

    let float = Point { x: 89.76, y: 87 };

    let student = Point{ x:"Peter Griffins", y: 78};

    println!("student name is:{:?} with marks of {:?}", student.x, student.y);

    println!("Integer point: {:?}", integer);
    println!("flaot point : {:?}", float);

    let nums = vec![890, 678, 4321, 9000];
    let res = largest(&nums);
    let letters = vec!["u", "a","z"];
    let i = largest(&letters);

    println!("lagest number is: {res}");
    println!("largest letter is: {i}");

}
