mod reindeergrid;
mod reindeergrid2;

use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    let grid = reindeergrid2::ReindeerGrid2::from(lines);
    grid.get_lowest_score()
}

fn solve2(lines: &Vec<String>) -> u64 {
    let grid = reindeergrid2::ReindeerGrid2::from(lines);
    grid.get_num_coords_on_lowest()
}

fn main() {
    match filereader::read_file("./day16/resources/input.txt") {
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
