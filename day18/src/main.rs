mod bytespace;

use crate::bytespace::ByteSpace;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    let mut byte_space = ByteSpace::from(lines);

    byte_space.bfs(0, 1024) as u64
}

fn solve2(lines: &Vec<String>) -> (usize, usize) {
    let mut byte_space = ByteSpace::from(lines);

    byte_space.find_blocking_byte()
}

fn main() {
    match filereader::read_file("./day18/resources/input.txt") {
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
