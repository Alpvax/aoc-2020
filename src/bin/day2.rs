#[macro_use]extern crate lazy_static;

use rust_2020::read_lines;
use regex::Regex;

fn main() {
    let results: Vec<_> = read_lines("puzzle-input/day2.txt").map(|l| parse_pwd(&l)).collect();
    println!("Part 1: {}\nPart 2: {}", results.iter().filter(|r| r.0).count(), results.iter().filter(|r| r.1).count());
}

fn parse_pwd(line: &str) -> (bool, bool) {
    lazy_static!{
        static ref PATTERN: Regex = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s([a-z]+)").unwrap();
    }
    let caps = PATTERN.captures(line).unwrap();
    let s = caps.get(1).unwrap().as_str().parse().unwrap();
    let e = caps.get(2).unwrap().as_str().parse().unwrap();
    let c = caps.get(3).unwrap().as_str().chars().next().unwrap();
    let pwd = caps.get(4).unwrap().as_str();
    let mut pc = pwd.chars();
    (
        std::ops::RangeInclusive::new(s, e).contains(&pwd.chars().filter(|pc| pc == &c).count()),
        (pc.nth(s - 1).unwrap() == c) != (pc.nth(e - s - 1).unwrap() == c)
    )
}