mod guard_grid;

use crate::guard_grid::{GuardGrid, GuardWalkResultType};
use common::filereader;
use std::cmp::PartialEq;

fn solve1(lines: &Vec<String>) -> u32 {
    let mut guard_grid = GuardGrid::from(lines);

    guard_grid.walk_guard().visited_cells.len() as u32
}

fn solve2(lines: &Vec<String>) -> u32 {
    let mut num_obstacles_that_loop = 0;

    // First, walk the guard normally to get all the cells it visits
    let mut guard_grid = GuardGrid::from(lines);
    let initial_position = guard_grid.guard_position.clone();
    let initial_direction = guard_grid.guard_direction.clone();

    // Next, for each cell it visited (aside from the starting cell), put an obstacle in the path
    for cell_coords in guard_grid.walk_guard().visited_cells {
        if cell_coords == initial_position {
            continue;
        }

        guard_grid.reset(initial_position.clone(), initial_direction);

        guard_grid.place_obstacle(&cell_coords);

        let walked_result = guard_grid.walk_guard();

        if walked_result.result_type == GuardWalkResultType::StuckInLoop {
            num_obstacles_that_loop += 1;
        }

        guard_grid.remove_obstacle(&cell_coords);
    }

    num_obstacles_that_loop
}

fn main() {
    match filereader::read_file("./day6/resources/input.txt") {
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
