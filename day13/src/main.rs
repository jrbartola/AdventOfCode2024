mod equations;

use crate::equations::Equations;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    lines.chunks(4).fold(0, |acc, chunk| {
        let equations = Equations::from(chunk);
        acc + {
            if let Some((a, b)) = equations.solve() {
                3 * a + b
            } else {
                0
            }
        }
    })
}

fn solve2(lines: &Vec<String>) -> u64 {
    lines.chunks(4).fold(0, |acc, chunk| {
        let equations = Equations::from_with_correction(chunk);
        acc + {
            if let Some((a, b)) = equations.solve() {
                3 * a + b
            } else {
                0
            }
        }
    })
}

fn main() {
    match filereader::read_file("./day13/resources/input.txt") {
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
