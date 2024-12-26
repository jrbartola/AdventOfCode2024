mod cpurace;

use crate::cpurace::CpuRace;
use common::filereader;
use std::collections::HashMap;

use common::coordinate::Coordinate;
use rayon::prelude::*;

fn run_cheats(mut cpu_race: CpuRace) -> Vec<u32> {
    let mut result_times = Vec::new();

    for r in 0..cpu_race.grid.row_len() {
        for c in 0..cpu_race.grid.col_len() {
            let coord = Coordinate(r, c);
            if cpu_race.grid.get(&coord).unwrap() != &'#' {
                continue;
            }

            println!("Cheating with {:?}", coord);

            cpu_race.grid.set(&coord, '.');

            result_times.push(cpu_race.bfs());

            cpu_race.grid.set(&coord, '#');
        }
    }

    result_times
}

fn solve1(lines: &Vec<String>) -> u32 {
    let cpurace1 = CpuRace::from(lines);
    let cpurace2 = CpuRace::from(lines);

    let slowest_time = cpurace1.bfs();

    run_cheats(cpurace2)
        .par_iter()
        .filter(|&&time| time < slowest_time && slowest_time - time >= 100)
        .count() as u32
}

fn solve2(lines: &Vec<String>) -> u32 {
    let cpu_race = CpuRace::from(lines);

    let dists = cpu_race.bfs_distances();
    let max_dist = *dists.get(&cpu_race.get_end()).unwrap();

    let results = cpu_race.apply_cheats(dists);
    let mut results_as_vec = results
        .iter()
        .map(|(k, v)| (max_dist - k, v))
        .collect::<Vec<_>>();
    results_as_vec.sort();

    println!("{:?}", results_as_vec);

    println!(
        "{:?}",
        results
            .iter()
            // .filter(|&(score, num)| max_dist - score >= 50)
            .collect::<HashMap<_, _>>()
            .values()
            .map(|&&num| num)
            .sum::<u32>()
    );

    0
}

fn main() {
    match filereader::read_file("./day20/resources/input.txt") {
        Ok(lines) => {
            // let start = std::time::Instant::now();
            // let result = solve1(&lines);
            // let duration = start.elapsed();
            // println!("Part 1: {:?} (took {:?})", result, duration);

            let start = std::time::Instant::now();
            let result = solve2(&lines);
            let duration = start.elapsed();
            println!("Part 2: {:?} (took {:?})", result, duration);
        }
        Err(e) => panic!("{}", e),
    }
}
