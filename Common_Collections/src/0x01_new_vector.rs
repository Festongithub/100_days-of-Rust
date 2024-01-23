#!/usr/bin/rustc

// create a new vector
//
// lets create a function that takes a vector as an argument
// & and [] points to the reference of the element

fn print_integers(v : &Vec<i32>)
{
    for &i in v {
        println!("{}", i*i);
    }
}

fn second_value(v: &Vec<i32>)
{
    println!("Value is {} ", &v[0]);
}
fn main()
{
let mut nums = Vec::new();
let a = [89, 54, 32, 56, 88, 34];

nums.push(a[5]);
nums.push(a[3]);
nums.push(a[4]);
nums.push(a[2]);
nums.push(a[1]);
nums.push(a[0]);

let second: &i32 = &nums[3];
println!("The second is {}", second);


for i in nums {
    println!("{}", i*i);
}

let marks = vec![85, 76, 90, 93, 97,80];
print_integers(&marks);

second_value(marks);
}
