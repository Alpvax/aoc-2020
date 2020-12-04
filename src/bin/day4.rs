#[macro_use]
extern crate lazy_static;

use regex::Regex;
use rust_2020::read_lines;
use std::{collections::HashMap, ops::RangeInclusive, rc::Rc};

type PassList = Vec<Rc<HashMap<String, String>>>;

fn main() {
    let lines = read_lines("puzzle-input/day4.txt");

    let mut passports1: PassList = Vec::new();
    let mut passports2: PassList = Vec::new();
    let mut current = HashMap::new();
    for line in lines {
        if line == "" {
            add_valid(&mut passports1, &mut passports2, current);
            current = HashMap::new();
        } else {
            for field in line.split_whitespace() {
                current.insert((&field[0..3]).to_string(), (&field[4..]).to_string());
            }
        }
    }
    if !current.is_empty() {
        add_valid(&mut passports1, &mut passports2, current);
    }
    println!("Part 1: {}\nPart 2: {}", passports1.len(), passports2.len());
}

fn add_valid(p1: &mut PassList, p2: &mut PassList, current: HashMap<String, String>) {
    let c = Rc::new(current);
    if validate_passport1(Rc::clone(&c)) {
        p1.push(Rc::clone(&c));
    }
    if validate_passport2(Rc::clone(&c)) {
        p2.push(Rc::clone(&c));
    }
}

fn validate_passport1(pass: Rc<HashMap<String, String>>) -> bool {
    for field in vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if pass.get(field).is_none() {
            return false;
        }
    }
    true
}

fn validate_passport2(pass: Rc<HashMap<String, String>>) -> bool {
    lazy_static! {
        static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref HGT: Regex = Regex::new(r"^(\d{2})in|(\d{3})cm$").unwrap();
        static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    check_range(&pass, "byr", 1920..=2002)
        && check_range(&pass, "iyr", 2010..=2020)
        && check_range(&pass, "eyr", 2020..=2030)
        && match pass.get("hgt") {
            None => false,
            Some(s) => match HGT.captures(s) as Option<regex::Captures> {
                None => false,
                Some(caps) => {
                    if let Some(i) = caps.get(1) {
                        (59..=76).contains(&i.as_str().parse::<i32>().unwrap())
                    } else {
                        (150..=193).contains(
                            &caps
                                .get(2)
                                .unwrap()
                                .as_str()
                                .parse::<i32>()
                                .expect("Not an Int"),
                        )
                    }
                }
            },
        }
        && match pass.get("hcl") {
            None => false,
            Some(s) => HCL.is_match(s),
        }
        && match pass.get("ecl") {
            None => false,
            Some(s) => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .any(|&c| c == s),
        }
        && match pass.get("pid") {
            None => false,
            Some(s) => PID.is_match(s),
        }
}

fn check_range(pass: &HashMap<String, String>, field: &str, range: RangeInclusive<u32>) -> bool {
    match pass.get(field) {
        Some(s) => range.contains(&s.parse::<u32>().unwrap()),
        None => false,
    }
}
