#!/usr/bin/rustc

use std::env;
use std::fs;

fn main()
{
    let args: Vec<String> = env::args().collect();

    fn parse_config(args: &[String])->(&str, &str) {
        let query = &args[1];
        let filename = &args[2];

        (query, filename)
    }
}
