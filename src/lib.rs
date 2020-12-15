//use std::env;
use std::fs;
use std::io::{self, BufRead};

pub mod grid;

pub fn parse_file(fname: &str) -> String {
    //let args: Vec<String> = env::args().collect();
    fs::read_to_string(fname).expect("Something went wrong reading the file")
}

pub fn read_lines(fname: &str) -> Box<dyn Iterator<Item = String>> {
    let file = fs::File::open(fname).expect("Something went wrong reading the file");
    Box::new(io::BufReader::new(file).lines().map(|l| l.unwrap()))
}
