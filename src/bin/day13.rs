use rust_2020::parse_file;
use std::convert::TryFrom;

fn main() {
    let lines = parse_file("puzzle-input/day13.txt")
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let earliest: u32 = lines[0].parse().unwrap();
    let part1 = lines[1]
        .split(',')
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse::<u32>().unwrap())
            }
        })
        .filter(|o| o.is_some())
        .map(|o| {
            let inc = o.unwrap();
            let mut next = 0;
            loop {
                next += inc;
                if next >= earliest {
                    return (inc, next - earliest);
                }
            }
        })
        .min_by_key(|(_, wait)| wait.clone())
        .map(|(id, wait)| id * wait)
        .unwrap();
    //let part2 = part2(&lines[1]);
    let part2 = part2("7,13,x,x,59,x,31,19");
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}

fn part2(line: &str) -> u64 {
    let mut timestamp = 0;
    let mut iter = line
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (s.parse::<u64>().unwrap(), u8::try_from(i).unwrap()));
    let inc: u64 = iter.next().unwrap().0.into();
    let busses = iter.collect();
    println!("Busses: {:?}", busses);
    while !check_timestamp(timestamp, &busses) {
        timestamp += inc;
    }
    timestamp
}

fn check_timestamp(timestamp: u64, checks: &Vec<(u64, u8)>) -> bool {
    println!("Timestamp: {}", timestamp);
    for (bus, offset) in checks {
        println!(
            "\t + {} % {} = {}",
            offset,
            bus,
            (timestamp + u64::from(*offset)) % bus
        );
        if (timestamp + u64::from(*offset)) % bus != 0 {
            return false;
        }
    }
    true
}
