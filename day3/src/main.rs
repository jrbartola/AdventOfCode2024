mod program;

use crate::program::Program;
use common::filereader;

fn eval_mul(num1: &str, num2: &str) -> u32 {
    let num1 = num1.parse::<u32>().unwrap();
    let num2 = num2.parse::<u32>().unwrap();

    num1 * num2
}

fn solve1(lines: &Vec<String>) -> u32 {
    let program = Program::new(lines.clone());

    program.run()
}

fn solve2(lines: &Vec<String>) -> u32 {
    let program = Program::new(lines.clone());

    program.run_with_conditionals()
}

fn main() {
    match filereader::read_file("./day3/resources/input.txt") {
        Ok(lines) => {
            let result = solve1(&lines);
            println!("Part 1: {:?}", result);

            let result = solve2(&lines);
            println!("Part 2: {:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
