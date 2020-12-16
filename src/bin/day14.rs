use regex::Regex;
use rust_2020::read_lines;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct DockMaskV1 {
    and_mask: u64, // convert bit to 0
    or_mask: u64,  //convert bit to 1
}
impl DockMaskV1 {
    fn new() -> Self {
        Self {
            and_mask: 0xF_FFFF_FFFF,
            or_mask: 0,
        }
    }
    fn apply(&self, val: u64) -> u64 {
        val & self.and_mask | self.or_mask
    }
}
impl std::str::FromStr for DockMaskV1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = Self::new();
        for (i, is1) in s
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, c)| *c != 'X')
            .map(|(i, c)| (1u64 << i, c == '1'))
        {
            match is1 {
                true => mask.or_mask |= i,
                false => mask.and_mask ^= i,
            }
        }
        Ok(mask)
    }
}

#[derive(Debug)]
struct FloatingMask {
    pos_mask: u64,
    masks: HashSet<u64>,
}
impl FloatingMask {
    fn new() -> Self {
        let pos_mask = !0xF_FFFF_FFFF;
        let mut masks = HashSet::new();
        masks.insert(pos_mask);
        Self {
            pos_mask, //Set all higher bits to 1, when flipped will become 0
            masks,
        }
    }
    fn _set_pos_floating(&mut self, pos: u64) {
        self.set_floating(1 << pos);
    }
    fn set_floating(&mut self, mask: u64) {
        self.pos_mask |= mask;
        self.masks = self
            .masks
            .iter()
            .flat_map(|m| vec![m | mask, *m].into_iter())
            .collect();
    }
    fn apply(&self, num: u64) -> HashSet<u64> {
        let n = num & !self.pos_mask;
        self.masks.iter().map(|m| n | m).collect()
    }
}

#[derive(Debug)]
struct DockMaskV2 {
    or_mask: u64, //convert bit to 1
    floating: FloatingMask,
}
impl DockMaskV2 {
    fn new() -> Self {
        Self {
            or_mask: 0,
            floating: FloatingMask::new(),
        }
    }
    fn apply(&self, val: u64) -> HashSet<u64> {
        let v = val | self.or_mask;
        self.floating.apply(v)
    }
}
impl std::str::FromStr for DockMaskV2 {
    type Err = (char, u64);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = Self::new();
        for (i, c) in s
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, c)| *c != '0')
            .map(|(i, c)| (1u64 << i, c))
        {
            match c {
                '1' => mask.or_mask |= i,             //Set to 1
                'X' => mask.floating.set_floating(i), // Set floating
                _ => return Err((c, i)),
            }
        }
        Ok(mask)
    }
}

fn _sample1() {
    let mask: DockMaskV1 = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();
    /*mem[8] = 11
    mem[7] = 101
    mem[8] = 0*/
    println!(
        "Mask: {:?}\n11 -> {}\n101 -> {}\n0 -> {}",
        mask,
        mask.apply(11),
        mask.apply(101),
        mask.apply(0)
    );
}

fn _sample2() {
    let mask: DockMaskV2 = "000000000000000000000000000000X1001X".parse().unwrap();
    /*mem[8] = 11
    mem[7] = 101
    mem[8] = 0*/
    println!("Mask: {:?}\n42 -> {:?}", mask, mask.apply(42),);
}

fn _debug_floating() {
    let mut f = FloatingMask::new();
    f._set_pos_floating(1);
    f._set_pos_floating(3);
    f._set_pos_floating(4);
    f._set_pos_floating(5);
    println!(
        "{:?} {{\n\t0-mask:\t{:b}\n\t\t{}\n}}",
        f,
        (f.pos_mask | 0x0_0000_0000),
        f.masks
            .iter()
            .map(|m| format!("{:b}", m & 0xF_FFFF_FFFF))
            .collect::<Vec<_>>()
            .join("\n\t\t")
    );
}

fn main() {
    let pattern = Regex::new(r"mem\[(\d+)\] = (\d+)|mask = ([\dX]+)").unwrap();
    let mut mask1 = DockMaskV1::new();
    let mut mask2 = DockMaskV2::new();
    let mut mem1: HashMap<u64, u64> = HashMap::new();
    let mut mem2: HashMap<u64, u64> = HashMap::new();
    for line in read_lines("puzzle-input/day14.txt") {
        let caps = pattern.captures(&line).unwrap();
        if let Some(m) = caps.get(3) {
            mask1 = m.as_str().parse().unwrap();
            mask2 = m.as_str().parse().unwrap();
        } else {
            let key = caps.get(1).unwrap().as_str().parse().unwrap();
            let val = caps.get(2).unwrap().as_str().parse().unwrap();
            mem1.insert(key, mask1.apply(val));
            for k in mask2.apply(key) {
                mem2.insert(k, val);
            }
        }
    }
    println!(
        "Part 1: {}\nPart 2: {}",
        mem1.values().fold(0u64, |sum, v| sum + u64::from(*v)),
        mem2.values().fold(0u64, |sum, v| sum + u64::from(*v)),
    );
}
