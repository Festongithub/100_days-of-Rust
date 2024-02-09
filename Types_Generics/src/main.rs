#!/usr/bin/rustc


fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut alpha = &list[0];

    for item in list {
        if item > alpha {
            alpha = item;
        }
    }
    alpha
}

fn smallest_char(list: &[char]) -> &char {
    let mut small = &list[0];

    for letters in list {
        if letters < small {
            small = letters;
        }

    }
    small
}


fn main()
{
    let numbers = vec![567, 890, 543, 1200];

    let res = largest(&numbers);
    println!("{} is the largest number", res);

    let letters = vec!['a', 'b', 'd', 'w'];

    println!("{} is the largest", largest_char(&letters));
    println!("{} is the smallest",smallest_char(&letters)); 
}
