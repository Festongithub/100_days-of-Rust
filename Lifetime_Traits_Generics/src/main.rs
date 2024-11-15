fn largest<T: std::cmp::PartialOrd>(array: &[T]) -> &T {
    let mut i = &array[0];

    for item in array {
        if item > i {
            i = item;
        }
    }
    i
}


struct Point<T, U> {
    x:T,
    y:U,
}
 


fn main() {
    let numbers = vec![8900, 9000, 3214, 5678, 87654];
    let j = largest(&numbers);

    println!("The largest number is {j}");

    let names = vec!["John", "Festus", "Georginah", "Gerald", "Nicholas"];

    let c = largest(&names);

    println!("Longest name is : {c}");

    let integer = Point{ x: 5, y: 10.80};

    println!("Float point x and y : {} , {}", integer.y, integer.x);
}
