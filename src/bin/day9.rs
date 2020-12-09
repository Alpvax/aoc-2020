use itertools::Itertools;
use rust_2020::read_lines;

const CHECK_LENGTH: usize = 25;

fn main() {
    let mut iter = read_lines("puzzle-input/day9.txt").map(|line| line.parse::<u64>().unwrap());
    let mut saved = Vec::new();
    for _ in 0..CHECK_LENGTH {
        saved.push(iter.next().unwrap());
    }
    let mut result = 0;
    for num in iter {
        saved.push(num);
        if result == 0 && !check(&saved) {
            result = num;
        }
    }
    println!("Part 1: {}\nPart 2: {}", result, part2(&saved, result));
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

fn part2(numbers: &Vec<u64>, target: u64) -> u64 {
    for (i, num) in numbers.iter().enumerate() {
        let mut total = *num;
        for &n in numbers[(i + 1)..].iter() {
            if n < target {
                total += n;
                if total < target {
                    continue;
                }
                if total == target {
                    return num + n;
                }
            }
            break;
        }
    }
    panic!("No contigous region found!");
}
