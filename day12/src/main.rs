mod flowergrid;

use crate::flowergrid::FlowerGrid;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u32 {
    let grid = FlowerGrid::from(lines);
    grid.get_price()
}

fn solve2(lines: &Vec<String>) -> u32 {
    let grid = FlowerGrid::from(lines);
    grid.get_discouted_price()
}

fn main() {
    match filereader::read_file("./day12/resources/input.txt") {
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
