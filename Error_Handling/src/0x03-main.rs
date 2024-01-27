#!/usr/bin/rustc

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _my_file = File::open("Hello.c").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello.c").unwrap_or_else(|error|{
                panic!("Problem creating the file: {:?}",error);
            })
        }else {
            panic!("Problem opening the file: {:?}",error);
        }
    });
}
