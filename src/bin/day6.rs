use rust_2020::read_lines;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut total1: usize = 0;
    let mut total2: usize = 0;
    let mut current1 =  HashSet::new();
    let mut current2: Option<HashSet<char>> = None;
    for line in read_lines("puzzle-input/day6.txt") {
        if line == "" {
            total1 += current1.len();
            current1 = HashSet::new();
            if current2.is_some() {
                total2 += current2.unwrap().len();
                current2 = None;
            }
        } else {
            for c in line.chars() {
                current1.insert(c);
            }
            match current2 {
                None => {
                    current2 = Some(HashSet::from_iter(line.chars()));
                },
                Some(ref mut set) => {
                    (*set).retain(|&c| line.contains(c));
                },
            }
        }
    }
    total1 += current1.len();
    total2 += if let Some(set) = current2 { set.len() } else { 0 };
    println!("Part 1: {}\nPart 2: {}", total1, total2);
}