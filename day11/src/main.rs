mod stonebucket;

use crate::stonebucket::StoneBucket;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    let mut stone_bucket = StoneBucket::from(lines[0].clone());

    for _ in 0..25 {
        stone_bucket.blink();
    }

    stone_bucket.len()
}

fn solve2(lines: &Vec<String>) -> u64 {
    let mut stone_bucket = StoneBucket::from(lines[0].clone());

    for _ in 0..75 {
        stone_bucket.blink();
    }

    stone_bucket.len()
}

fn main() {
    match filereader::read_file("./day11/resources/input.txt") {
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
