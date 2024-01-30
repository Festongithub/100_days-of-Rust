#!/usr/bin/rustc

use std::fs::File;
use std::io::ErrorKind;

fn main()
{
    let user_file = File::open("Users.txt").unwrap_or_else(|error| {
if error.kind() == ErrorKind::NotFound {
File::create("Users.txt").unwrap_or_else(|error|{
panic!("Users file not found: {:?}", error);
})
}
else{
panic!("Problem opening the file: {:?}", error);
}
});
}
