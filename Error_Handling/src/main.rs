#!/usr/bin/rustc

use std::fs::File;
use std::io::ErrorKind;

enum Result <T, E>
{
Ok(T),
Err(E),
}
fn main()
{

let my_file = File::open("my_file.txt");

let check_file = match my_file {
Ok(file) => file,
Err(error) => match error.kind()
{
ErrorKind::NotFound => {
match File::create("my_file.txt")
{
Ok(fc) => fc,
Err(e) => panic!("Problem creating the file{:?}", e),
}
}
other_error => {
panic!("Problem opening the file: {:?}", other_error);
}
},
};
}
