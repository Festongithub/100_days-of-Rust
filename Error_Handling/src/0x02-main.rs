#!/usr/bin/rustc

use std::fs::File;
use std::io::ErrorKind;

fn main()
{
    let check_file = File::open("Check.py");

    let _my_check  = match check_file {
        Ok(file) => file,
        Err(error) => match error.kind()
        {
            ErrorKind::NotFound => {
                match File::create("Check.py")
                {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating file: {:?}", e),
                }
            }
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        },
    };
}
