use std::fs::File;
use std::io::{self,  Read, ErrorKind};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("Rust.rs")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {

    let _ = read_username_from_file();

    let _getfiles = File::open("hello.txt");

    let _newfile = match _getfiles{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    let _pythonfiles = File::open("hello.py").expect("hello.py should be included");
}
