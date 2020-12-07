use rust_2020::read_lines;
use std::convert::TryFrom;
use std::ops::Range;

enum HalfSelect {
    Low,
    High,
}

#[derive(Debug)]
enum Half {
    Range(Range<u8>),
    Value(u8),
}

impl Half {
    fn range(&self) -> Range<u8> {
        match self {
            Self::Range(r) => r.start..r.end,
            Self::Value(v) => *v..(v + 1),
        }
    }
    fn value(&self) -> u8 {
        match self {
            Self::Range(_) => panic!("Range does not have single value!"),
            Self::Value(v) => *v,
        }
    }
}

fn lookup_half(range: Range<u8>, half: HalfSelect) -> Half {
    let delta = u8::try_from(range.len()).expect("Range too large: bigger than u8") / 2;
    let res = match half {
        HalfSelect::Low => range.start..(range.start + delta),
        HalfSelect::High => (range.start + delta)..range.end,
    };
    if delta == 1 {
        Half::Value(res.start)
    } else {
        Half::Range(res)
    }
}

fn lookup_seat(data: &str) -> (u8, u8) {
    let mut row = Half::Range(0..128);
    let mut seat = Half::Range(0..8);
    for c in data.chars() {
        match c {
            'F' => row = lookup_half(row.range(), HalfSelect::Low),
            'B' => row = lookup_half(row.range(), HalfSelect::High),
            'L' => seat = lookup_half(seat.range(), HalfSelect::Low),
            'R' => seat = lookup_half(seat.range(), HalfSelect::High),
            _ => panic!("Invalid character!"),
        }
    }
    (row.value(), seat.value())
}

fn calculate_id(data: (u8, u8)) -> u32 {
    u32::from(data.0) * 8 + u32::from(data.1)
}

fn main() {
    //println!("FBFBBFFRLR: {:?}", lookup_seat("FBFBBFFRLR"));
    let data = read_lines("puzzle-input/day5.txt")
        .map(|line| lookup_seat(&line))
        .map(calculate_id)
        .collect::<std::collections::HashSet<u32>>();
    let max = data.iter().max().unwrap().clone();
    let missing = (data.iter().min().unwrap().clone()..max)
        .filter(|i| {
            !data.contains(i) && *i > 0 && data.contains(&(i - 1)) && data.contains(&(i + 1))
        })
        .next()
        .unwrap();
    println!("Part 1: {}\nPart 2: {}", max, missing);
}
