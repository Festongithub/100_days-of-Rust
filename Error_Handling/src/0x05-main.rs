#!/usr/bin/rustc

use std::fs::File;
use std::io::{self, Read};

fn main()
{
    fn read_username() -> Result<String, io::Error>
    {
        let mut username_file = File::open("User.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
}
