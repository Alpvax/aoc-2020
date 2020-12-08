use core::panic;

use rust_2020::read_lines;

#[derive(Debug)]
enum Operator {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

enum ProgramResult {
    Loop(i32),
    Finish(i32),
}

impl ProgramResult {
    fn value(self) -> i32 {
        match self {
            ProgramResult::Loop(acc) => acc,
            ProgramResult::Finish(acc) => acc,
        }
    }
}

fn main() {
    let instructions: Vec<_> = read_lines("puzzle-input/day8.txt")
        .map(|line| parse_op(&line))
        .collect();
    let mut accumulator = 0;
    for i in instructions
        .iter()
        .enumerate()
        .filter(|(_, op)| match op {
            Operator::Acc(_) => false,
            _ => true,
        })
        .map(|(i, _)| i)
    {
        match run(&instructions, i) {
            ProgramResult::Loop(_) => (),
            ProgramResult::Finish(acc) => {
                accumulator = acc;
                break;
            }
        }
    }
    println!(
        "Part 1: {}\nPart 2: {}",
        run(&instructions, instructions.len()).value(),
        accumulator
    );
}

fn run(instructions: &Vec<Operator>, switch_index: usize) -> ProgramResult {
    let target_index = instructions.len();
    let mut run_indexes = std::collections::HashSet::new();
    let mut index: usize = 0;
    let mut accumulator = 0;
    loop {
        if run_indexes.contains(&index) {
            return ProgramResult::Loop(accumulator);
        }
        if index == target_index {
            return ProgramResult::Finish(accumulator);
        }
        run_indexes.insert(index);
        match &instructions[index] {
            Operator::Acc(amount) => {
                accumulator += amount;
                index += 1;
            }
            Operator::Jmp(amount) => {
                if index == switch_index {
                    index += 1; //Nop
                } else {
                    index = add_index(index, amount);
                }
            }
            Operator::Nop(amount) => {
                if index == switch_index {
                    index = add_index(index, amount); //Jmp
                } else {
                    index += 1;
                }
            }
        }
    }
}

fn add_index(index: usize, amount: &i32) -> usize {
    if amount < &0 {
        index.checked_sub(amount.abs() as usize)
    } else {
        index.checked_add(amount.abs() as usize)
    }
    .unwrap()
}

fn parse_op(line: &str) -> Operator {
    let amount: i32 = line[4..].parse().unwrap();
    match &line[..3] {
        "acc" => Operator::Acc(amount),
        "jmp" => Operator::Jmp(amount),
        "nop" => Operator::Nop(amount),
        _ => panic!("Invalid operator"),
    }
}
