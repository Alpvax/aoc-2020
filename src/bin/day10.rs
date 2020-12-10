use itertools::Itertools;
use rust_2020::read_lines;
use std::collections::HashMap;

struct Acc {
    data: HashMap<u16, u8>,
    previous: u16,
    values: HashMap<u16, u64>,
}
impl Acc {
    fn new() -> Acc {
        let mut values = HashMap::new();
        values.insert(0, 1);
        Acc {
            data: HashMap::new(),
            previous: 0,
            values,
        }
    }
    fn push(mut self, num: u16) -> Self {
        *self.data.entry(num - self.previous).or_insert(0) += 1;
        self.previous = num;
        self.update_paths(num);
        self
    }
    fn end(mut self) -> (u32, u64) {
        *self.data.entry(3).or_insert(0) += 1;
        self.previous += 3;
        self.update_paths(self.previous);
        (
            self.num_increments(1) * self.num_increments(3),
            *self.values.get(&self.previous).unwrap(),
        )
    }
    fn num_increments(&self, inc: u16) -> u32 {
        u32::from(*self.data.get(&inc).unwrap())
    }
    fn update_paths(&mut self, num: u16) {
        let count = ((if num >= 3 {
            num - 3
        } else if num >= 2 {
            num - 2
        } else if num >= 1 {
            num - 1
        } else {
            0
        })..num)
            .map(|n| self.values.get(&n).unwrap_or(&0))
            .sum();
        self.values.insert(num, count);
    }
}

fn main() {
    let data = read_lines("puzzle-input/day10.txt")
        .map(|line| line.parse::<u16>().unwrap())
        .sorted()
        .fold(Acc::new(), |acc, num| acc.push(num))
        .end();
    println!("Part 1: {}\nPart 2: {}", data.0, data.1);
}
