use regex::Regex;
use rust_2020::read_lines;
use std::collections::{HashMap, HashSet};

fn main() {
    let name_re: Regex = Regex::new(r"^([a-z ]+) bags").unwrap();
    let contents_re: Regex = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();
    //let mut data = HashMap::new();
    let mut parents = HashMap::new();
    let mut children = HashMap::new();
    for line in read_lines("puzzle-input/day7.txt") {
        let name = name_re.captures(&line).unwrap().get(1).unwrap().as_str();
        let c = children.entry(name.to_string()).or_insert(HashMap::new());
        for (bag, count) in contents_re.captures_iter(&line).map(|caps| {
            (
                caps.get(2).unwrap().as_str().to_string(),
                caps.get(1)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse::<usize>()
                    .unwrap(),
            )
        }) {
            parents
                .entry(bag.to_string())
                .or_insert(HashSet::new())
                .insert(name.to_string());
            c.insert(bag, count);
        }
    }
    println!(
        "Part 1: {}\nPart 2: {}",
        count_parents(&parents, "shiny gold"),
        count_nested(&children, "shiny gold") - 1,
    );
}

fn count_parents(parents: &HashMap<String, HashSet<String>>, bag: &str) -> usize {
    let mut processed = HashSet::new();
    let mut to_process = HashSet::new();
    to_process.insert(bag.to_string());
    while to_process.len() > 0 {
        to_process = to_process
            .iter()
            .filter_map(|bag| {
                processed.insert(bag.to_string());
                parents
                    .get(&bag.to_string())
                    .map(|s| s.iter().map(|s| s.to_string()))
            })
            .flatten()
            .collect();
    }
    processed.len() - 1
}

fn count_nested(children: &HashMap<String, HashMap<String, usize>>, bag: &str) -> usize {
    let mut count = 1;
    if let Some(val) = children.get(bag) {
        for (child, num) in val {
            count += num * count_nested(children, child);
        }
    }
    count
}
