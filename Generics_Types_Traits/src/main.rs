fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn letters(list: &[char]) -> &char {
    let mut smallest = &list[0];

    for i in list {
        if i < smallest {
            smallest = i;
        }
    }
    smallest
}


fn main() {
    let numbers = vec![34, 56, 25, 100, 65];

    let words = vec!['a', 'x', 'y', 'z'];
    let res = letters(&words);
    let result = largest(&numbers);
    println!("The largest number is {result}");
    println!("The smallest letter is {res}");
}
