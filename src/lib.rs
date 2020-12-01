//use std::env;
use std::fs;
use std::io::{self, BufRead};

pub fn parse_file<T>(fname: &str, f: fn(&str) -> T) -> T {
    //let args: Vec<String> = env::args().collect();
    f(&fs::read_to_string(fname)
        .expect("Something went wrong reading the file")
    )
}

pub fn read_lines(fname: &str) -> Box<dyn Iterator<Item=String>> {
    let file = fs::File::open(fname).expect("Something went wrong reading the file");
    Box::new(io::BufReader::new(file).lines().map(|l| l.unwrap()))
}
