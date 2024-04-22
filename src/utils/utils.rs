#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn load_from_file(file_path: &str) -> Vec<String> {
    let mut contents: Vec<String> = Vec::new();
    
    let file = File::open(file_path).unwrap();
    BufReader::new(file).lines().for_each(|line| contents.push(line.unwrap()));
    
    contents
}