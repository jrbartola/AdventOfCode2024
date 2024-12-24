mod trailmap;

use common::filereader;

fn solve1(lines: &Vec<String>) -> u32 {
    let trailmap = trailmap::TrailMap::from(lines);
    trailmap.search_score()
}

fn solve2(lines: &Vec<String>) -> u32 {
    let trailmap = trailmap::TrailMap::from(lines);
    trailmap.search_rating()
}

fn main() {
    match filereader::read_file("./day10/resources/input.txt") {
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
