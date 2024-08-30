#!/usr/bin/rust

fn main()
{
    let i: u32 = 90;
    println!("{}", i * i);

    let rem = 43 % 5;
    println!("{}", rem);

    let c = 'z';
    println!("{}", c);

    let nums = (789, 432, 234, 567, 900);

    let (_v, _w, _x, _y, _z) = nums;

    println!("Value of z is: {_z}");

    let numbers = [345, 678, 432, 124, 567];
    println!("{}", numbers[0] * numbers[1]);
}
