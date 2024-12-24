mod equation;

use crate::equation::Equation;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    lines
        .iter()
        .map(|line| Equation::from_string(line))
        .filter(|e| e.is_solvable_no_concat())
        .map(|e| e.test_value)
        .sum()
}

fn solve2(lines: &Vec<String>) -> u64 {
    lines
        .iter()
        .map(|line| Equation::from_string(line))
        .filter(|e| e.is_solvable_with_concat())
        .map(|e| e.test_value)
        .sum()
}

fn main() {
    match filereader::read_file("./day7/resources/input.txt") {
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
