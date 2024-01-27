#!/usr/bin/rustc
use std::fs::File;
use std::io::ErrorKind;

fn main()
{
    let my_file = File::open("myfile.txt");

    let _my_file_result  = match my_file {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => {
                match File::create("myfile.txt")
                {
                    Ok(fc) => fc,
                    Err(e) => panic! ("Problem creating the file: {:?}",e),
                }
            }
            other_error => {
                panic!("Problem opening the file : {:?}", other_error);
            }
        }
    };
}
