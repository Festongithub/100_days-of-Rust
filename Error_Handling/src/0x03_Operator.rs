#!/usr/bin/rustc

use std::fs::File;
use std::io::{self,Read};

fn main()
{

let username_from_file() -> Result<String, io::Error>
{
let mut username_file = File::open("Tests.csv")?;
let mut username = String::new();
username_file.read_to_string(&mut username)?;
Ok(username);
}
}