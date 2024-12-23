use common::filereader;
use std::collections::HashMap;

fn compare_and_tally(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    if list1.len() != list2.len() {
        panic!("Lists are not the same length");
    }

    let mut tally = 0;

    for i in 0..list1.len() {
        if list1[i] > list2[i] {
            tally += list1[i] - list2[i];
        } else {
            tally += list2[i] - list1[i];
        }
    }

    tally
}

fn collect_lists(lines: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    // Each line is of the format: "\d{5}   \d{5}"
    lines.into_iter().for_each(|line| {
        let mut iter = line.split_whitespace();
        let num1: u32 = iter.next().unwrap().parse().unwrap();
        first_list.push(num1);

        let num2: u32 = iter.next().unwrap().parse().unwrap();
        second_list.push(num2);
    });

    (first_list, second_list)
}

fn list_to_counter(list: Vec<u32>) -> HashMap<u32, u32> {
    let mut counter = HashMap::new();

    list.into_iter().for_each(|num| {
        let count = counter.entry(num).or_insert(0);
        *count += 1;
    });

    counter
}

fn solve1(lines: &Vec<String>) -> u32 {
    let (mut first_list, mut second_list) = collect_lists(lines);

    first_list.sort();
    second_list.sort();

    compare_and_tally(first_list, second_list)
}

fn solve2(lines: &Vec<String>) -> u32 {
    let (first_list, second_list) = collect_lists(lines);

    let second_list_counter = list_to_counter(second_list);

    first_list.into_iter().fold(0, |acc, num| {
        let item_count = *second_list_counter.get(&num).unwrap_or(&0);

        acc + num * item_count
    })
}

fn main() {
    match filereader::read_file("./day1/resources/input.txt") {
        Ok(lines) => {
            let result = solve1(&lines);
            println!("Part 1: {:?}", result);

            let result = solve2(&lines);
            println!("Part 2: {:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
