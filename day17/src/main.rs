mod program;

use crate::program::Program;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    let mut program = Program::from(lines);
    let output = program.run();

    println!(
        "{}",
        output
            .into_iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    0
}

fn solve2(lines: &Vec<String>) -> u64 {
    // let mut register_a_value: u64 = 100_000_00_000_000;
    let mut register_a_value: u64 = 8u64.pow(15)
        + 29u64 * 8u64.pow(14)
        + 3 * 8u64.pow(13)
        + 2 * 8u64.pow(12)
        + 3 * 8u64.pow(11)
        + 7 * 8u64.pow(9)
        + 8u64.pow(8)
        + 3 * 8u64.pow(7)
        + 3 * 8u64.pow(6)
        + 2 * 8u64.pow(5)
        + 6 * 8u64.pow(4);

    let program = Program::from(lines);
    let mut output = Vec::new();

    while output != program.instructions {
        output = program.clone().with_register_a(register_a_value).run();

        println!(
            "{}: {}",
            register_a_value,
            output
                .iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );

        register_a_value += 1;
    }
    // 4 -> 6 -> 7 -> 0 -> 1 -> 2 -> 3
    register_a_value - 1
}

fn main() {
    match filereader::read_file("./day17/resources/input.txt") {
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
