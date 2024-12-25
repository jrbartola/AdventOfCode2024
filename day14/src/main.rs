mod fastrobotgrid;
mod robotgrid;

use crate::robotgrid::RobotGrid;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    let mut grid = RobotGrid::from(lines);

    for _ in 0..100 {
        grid.tick();
    }

    // grid.print();

    // Take product
    let (q1, q2, q3, q4) = grid.parse_quadrants();

    q1 as u64 * q2 as u64 * q3 as u64 * q4 as u64
}

fn solve2(lines: &Vec<String>) -> u64 {
    // let mut grid = FastRobotGrid::from(lines);

    // let (robots, raw_grid) = grid.tick(1000000);

    let mut grid = RobotGrid::from(lines);
    let mut min_safety = u64::MAX;
    let mut min_safety_tick = 0;

    for sec in 0..10000 {
        grid.tick();

        // Take product
        let (q1, q2, q3, q4) = grid.parse_quadrants();

        let safety = q1 as u64 * q2 as u64 * q3 as u64 * q4 as u64;

        if safety < min_safety {
            min_safety = safety;
            min_safety_tick = sec;
        }

        if sec - min_safety_tick > 5000 {
            break;
        }
    }

    min_safety_tick + 1
}

fn main() {
    match filereader::read_file("./day14/resources/input.txt") {
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
