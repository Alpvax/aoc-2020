use itertools::Itertools;
use rust_2020::read_lines;

const CHECK_LENGTH: usize = 25;

fn main() {
    let mut iter = read_lines("puzzle-input/day9.txt").map(|line| line.parse::<u64>().unwrap());
    let mut saved = Vec::new();
    for _ in 0..CHECK_LENGTH {
        saved.push(iter.next().unwrap());
    }
    for num in iter {
        saved.push(num);
        if !check(&saved) {
            println!("Part 1: {}", num);
            break;
        }
    }
}

fn check(numbers: &Vec<u64>) -> bool {
    let l = numbers.len() - 1;
    let val = numbers[l]; //last
    for pair in numbers[l - CHECK_LENGTH..l]
        .iter()
        .sorted()
        .filter(|&n| n < &val)
        .permutations(2)
    {
        if pair[0] + pair[1] == val {
            return true;
        }
    }
    false
}
