mod pagerule;
mod updates;

use crate::pagerule::{PageRule, RuleMap};
use crate::updates::Update;
use common::filereader;

fn parse_input(lines: &Vec<String>) -> (Vec<PageRule>, Vec<Update>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut is_rules = true;
    for line in lines {
        if line.is_empty() {
            is_rules = false;
            continue;
        }

        if is_rules {
            rules.push(PageRule::from_str(line.clone()));
        } else {
            updates.push(Update::from_str(line.clone()));
        }
    }

    (rules, updates)
}

fn solve1(lines: &Vec<String>) -> u32 {
    let (rules, updates) = parse_input(lines);

    let rulemap = RuleMap::from(rules.iter().collect());

    updates
        .into_iter()
        .filter(|update| update.is_valid_for_rulemap(&rulemap))
        .fold(0, |acc, update| acc + update.get_middle_page())
}

fn solve2(lines: &Vec<String>) -> u32 {
    let (rules, updates) = parse_input(lines);

    let rulemap = RuleMap::from(rules.iter().collect());

    updates
        .into_iter()
        .filter(|update| !update.is_valid_for_rulemap(&rulemap))
        .fold(0, |acc, mut update| {
            update.reorder(&rulemap);

            acc + update.get_middle_page()
        })
}

fn main() {
    match filereader::read_file("./day5/resources/input.txt") {
        Ok(lines) => {
            let start = std::time::Instant::now();
            let result = solve1(&lines);
            let duration = start.elapsed();
            println!("Part 1: {:?} (took {:?})", result, duration);

            let start = std::time::Instant::now();
            let result = solve2(&lines);
            let duration = start.elapsed();
            println!("Part 2: {:?} (took {:?})", result, duration);
        }
        Err(e) => panic!("{}", e),
    }
}
