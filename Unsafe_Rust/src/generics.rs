fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {

    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn smallest<T: std::cmp::PartialOrd>(numbers: &[T]) -> &T {
    let mut small = &numbers[0];

    for i in numbers {
        if i < small {
            small = i;
        }
    }
    small
}


struct Point<T> {
    x: T,
    y: T,
}



fn main() {
    let nums = vec![890, 432, 1234, 6789];
    let letters = ["a","b", "x"];

    let l = largest(&nums);
    let s = smallest(&nums);

    let a = largest(&letters);
    let b = smallest(&letters);


    println!("Largest number is: {}", l);
    println!("smallest number is: {}", s);

    println!("Largest letter is: {}", a);
    println!("smallest letters is :{}", b);

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 78.09, y: 90.23};

    println!("{}", float.x);
}
