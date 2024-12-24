mod filesystem;

use crate::filesystem::FileSystem;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    let mut fs = FileSystem::new(&lines[0]);

    fs.move_files_left();
    fs.get_checksum()
}

fn solve2(lines: &Vec<String>) -> u64 {
    let mut fs = FileSystem::new(&lines[0]);

    fs.move_whole_files_left();
    fs.get_checksum()
}

fn main() {
    match filereader::read_file("./day9/resources/input.txt") {
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
