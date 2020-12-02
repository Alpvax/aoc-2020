use itertools::Itertools;
use rust_2020::read_lines;

fn main() {
    let numbers: Vec<u32> = read_lines("puzzle-input/day1.txt")
        .map(|l| l.parse::<u32>().unwrap())
        .collect();
    for p in numbers.iter().permutations(2) {
        if p[0] + p[1] == 2020 {
            println!("Part 1: {}", p[0] * p[1]);
            break;
        }
    }
    for p in numbers.iter().permutations(3) {
        if p[0] + p[1] + p[2] == 2020 {
            println!("Part 2: {}", p[0] * p[1] * p[2]);
            break;
        }
    }
}
