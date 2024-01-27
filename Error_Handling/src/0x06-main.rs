#!/usr/bin/rustc
use std::fs::File;

fn main()
{
    let my_file = File::open("Files.txt")?;
}
