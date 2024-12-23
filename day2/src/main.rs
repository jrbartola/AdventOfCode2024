mod report;

use crate::report::Report;
use common::filereader;

fn to_report(line: &String) -> Report {
    let mut values = Vec::new();
    for num in line.split_whitespace() {
        values.push(num.parse().unwrap());
    }

    Report::new(values)
}

fn solve1(lines: &Vec<String>) -> u32 {
    lines
        .iter()
        .map(to_report)
        .filter(|report| report.is_valid(None))
        .count() as u32
}

fn solve2(lines: &Vec<String>) -> u32 {
    lines
        .iter()
        .map(to_report)
        .filter(|report| report.is_valid_dampener())
        .count() as u32
}

fn main() {
    match filereader::read_file("./day2/resources/input.txt") {
        Ok(lines) => {
            let result = solve1(&lines);
            println!("Part 1: {:?}", result);

            let result = solve2(&lines);
            println!("Part 2: {:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
