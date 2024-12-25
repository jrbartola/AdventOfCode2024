mod fatwearhouse;
mod warehouse;

use crate::fatwearhouse::FatWarehouse;
use crate::warehouse::Warehouse;
use common::filereader;

fn solve1(lines: &Vec<String>) -> u64 {
    let mut warehouse = Warehouse::from(lines);
    warehouse.run();

    warehouse.print();
    warehouse.compute_gps()
}

fn solve2(lines: &Vec<String>) -> u64 {
    let mut warehouse = FatWarehouse::from(lines);

    warehouse.run();
    warehouse.print();

    warehouse.compute_gps()
}

fn main() {
    match filereader::read_file("./day15/resources/input.txt") {
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
