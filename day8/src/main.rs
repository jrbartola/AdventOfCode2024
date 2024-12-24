mod antenna_grid;

use crate::antenna_grid::AntennaGrid;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u32 {
    let mut grid = AntennaGrid::new(lines);
    let antinodes = grid.generate_antinodes(1);
    antinodes.len() as u32
}

fn solve2(lines: &Vec<String>) -> u32 {
    let mut grid = AntennaGrid::new(lines);
    let antinodes = grid.generate_antinodes(9999);
    antinodes.len() as u32
}

fn main() {
    match filereader::read_file("./day8/resources/input.txt") {
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
