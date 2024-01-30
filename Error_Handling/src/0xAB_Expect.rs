#!/usr/bin/rustc

use std::fs::File;
fn main()
{
let my_file = File::open("Tests.py").expect("Tests.py be included in this directory");
}