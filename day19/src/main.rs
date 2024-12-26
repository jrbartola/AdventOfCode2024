mod onsen;

use crate::onsen::Onsen;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    let onsen = Onsen::from(lines);
    onsen.calculate_possible_designs() as u64
}

fn solve2(lines: &Vec<String>) -> u64 {
    let onsen = Onsen::from(lines);

    onsen.calculate_all_possible_designs() as u64
}

fn main() {
    match filereader::read_file("./day19/resources/input.txt") {
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
