#!/usr/bin/rustc

fn main()
{
let number_list = vec![34, 56, 78, 100, 65];

let mut largest = &number_list[1];
let mut smallest = &number_list[1];
for number in &number_list{
if number > largest {
largest = number;
}
}

for i in &number_list{
if i < smallest {
i = smallest;
}
}
println!("{} is the smallest", smallest);
println!("The largest number is {}", largest);
}
