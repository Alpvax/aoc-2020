use std::collections::HashMap;

struct NumIter {
    index: usize,
    last_indices: HashMap<usize, usize>,
    seed_values: Vec<usize>,
    next_val: usize,
}
impl NumIter {
    fn new(seed_values: Vec<usize>) -> Self {
        Self {
            index: 0,
            last_indices: HashMap::new(),
            seed_values: seed_values.clone(),
            next_val: seed_values[0],
        }
    }
}
impl Iterator for NumIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.next_val;
        self.next_val = if self.index + 1 < self.seed_values.len() {
            self.seed_values[self.index + 1]
        } else {
            match self.last_indices.get(&val) {
                Some(&i) => self.index - i,
                None => 0,
            }
        };
        self.last_indices.insert(val, self.index);
        self.index += 1;
        Some(val)
    }
}

fn main() {
    let mut iter = NumIter::new(vec![0, 1, 4, 13, 15, 12, 16]);
    println!("Part 1: {}", iter.nth(2019).unwrap()); // Split to output pt1 as soon as it's ready
    println!("Part 2: {}", iter.nth(30000000 - 2021).unwrap());
}
