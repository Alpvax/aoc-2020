use rust_2020::read_lines;

fn main() {
    let mut lines = read_lines("puzzle-input/day3.txt");
    let len = lines.next().unwrap().chars().count();
    let mut slope_1_1 = SlopeResult::new(1);
    let mut slope_3_1 = SlopeResult::new(3);
    let mut slope_5_1 = SlopeResult::new(5);
    let mut slope_7_1 = SlopeResult::new(7);
    let mut slope_1_2 = SlopeResult::new(1);
    slope_1_2.row_skip = 2;
    for (i, line) in lines.enumerate() {
        slope_1_1.next_row(&line, len, i + 1);
        slope_3_1.next_row(&line, len, i + 1);
        slope_5_1.next_row(&line, len, i + 1);
        slope_7_1.next_row(&line, len, i + 1);
        slope_1_2.next_row(&line, len, i + 1);
    }
    println!(
        "Part 1: {}\nPart 2: {}",
        slope_3_1.trees,
        slope_1_1.trees * slope_3_1.trees * slope_5_1.trees * slope_7_1.trees * slope_1_2.trees
    );
}

#[derive(Debug)]
struct SlopeResult {
    column: usize,
    trees: u32,
    col_delta: usize,
    row_skip: usize,
}

impl SlopeResult {
    fn new(col_delta: usize) -> SlopeResult {
        SlopeResult {
            column: 0,
            trees: 0,
            col_delta,
            row_skip: 1,
        }
    }
    fn next_row(&mut self, line: &str, len: usize, row: usize) {
        if row % self.row_skip == 0 {
            self.column = (self.column + self.col_delta) % len;
            if line.chars().nth(self.column).unwrap() == '#' {
                self.trees += 1;
                println!("Tree at ({}, {})", self.column, row);
            }
        }
    }
}
