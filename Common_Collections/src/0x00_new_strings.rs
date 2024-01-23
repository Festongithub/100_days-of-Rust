#!/usr/bin/rustc

fn create_string(s) -> String {
    println!("{}", String::from("s"));
}

fn main()
{
    let mut mess = String::new();
    let new_mess  = "Hello world";
    mess = new_mess.to_string();
    println!("{}", mess);
    create_string();
}
