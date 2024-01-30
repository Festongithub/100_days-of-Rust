#!/usr/bin/rustc

use std::fs::File;

fn main()
{
   let main_file = File::open("Text.txt").unwrap();
}