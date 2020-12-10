use itertools::Itertools;
use rust_2020::read_lines;
use std::collections::HashMap;

struct Acc {
    data: HashMap<u16, u8>,
    previous: u16,
}
impl Acc {
    fn new() -> Acc {
        Acc {
            data: HashMap::new(),
            previous: 0,
        }
    }
    fn push(mut self, num: u16) -> Self {
        *self.data.entry(num - self.previous).or_insert(0) += 1;
        self.previous = num;
        self
    }
    fn end(mut self) -> HashMap<u16, u8> {
        *self.data.entry(3).or_insert(0) += 1;
        self.data
    }
}

fn main() {
    let data = read_lines("puzzle-input/day10.txt")
        .map(|line| line.parse::<u16>().unwrap())
        .sorted()
        .fold(Acc::new(), |acc, num| acc.push(num))
        .end();
    println!(
        "Part 1: {}",
        u32::from(*data.get(&1).unwrap()) * u32::from(*data.get(&3).unwrap())
    );
}
