mod wordsearch;
mod xmaswordsearch;

use crate::wordsearch::WordSearch;
use crate::xmaswordsearch::XmasWordSearch;
use common::filereader;

const WORD: &str = "XMAS";
const WORD_PART_2: &str = "MAS";

fn solve1(lines: &Vec<String>) -> u32 {
    let word_search = WordSearch::new(lines.clone());
    word_search.solve(WORD)
}

fn solve2(lines: &Vec<String>) -> u32 {
    let word_search = XmasWordSearch::new(lines.clone());
    word_search.solve(WORD_PART_2)
}

fn main() {
    match filereader::read_file("./day4/resources/input.txt") {
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
