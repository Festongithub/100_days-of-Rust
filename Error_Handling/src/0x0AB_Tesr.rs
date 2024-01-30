#!/usr/bin/rustc

use std::fs::File;
use std::io::{self, Read};

// Handling ====> Errors in a file
// File ===> Handle
fn main()
{

fn read_username_from_file() -> Result<String, io::Error> {
let username_file = File::open("Customers.csv");

let mut username_result = match username_file {
Ok(file) => file,
Err(e) => return Err(e),
};

let mut username = String::new();

match username_result.read_to_string(&mut username)
{
Ok(_) => Ok(username),
Err(e) => Err(e),
}
}
}