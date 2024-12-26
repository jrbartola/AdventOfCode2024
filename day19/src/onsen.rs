use std::collections::{HashMap, HashSet};

pub struct Onsen {
    towels: HashSet<String>,
    designs: Vec<String>,
}

impl Onsen {
    pub fn from(lines: &Vec<String>) -> Self {
        Self {
            towels: lines[0].split(", ").map(|s| s.to_string()).collect(),
            designs: lines[2..]
                .iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        }
    }

    pub fn calculate_possible_designs(&self) -> u32 {
        self.designs.iter().fold(0, |acc, design| {
            if self.is_design_possible(design) {
                acc + 1
            } else {
                acc
            }
        })
    }

    pub fn calculate_all_possible_designs(&self) -> u64 {
        self.designs.iter().fold(0, |acc, design| {
            acc + self.num_ways_to_make_design(design, &mut HashMap::new())
        })
    }

    fn num_ways_to_make_design(&self, design: &str, cache: &mut HashMap<String, u64>) -> u64 {
        if design.is_empty() {
            return 1;
        }

        if let Some(&count) = cache.get(design) {
            return count;
        }

        let mut designs_to_check = Vec::new();

        for end_idx in 1..(design.len() + 1) {
            let towel = &design[0..end_idx];
            if self.towels.contains(towel) {
                designs_to_check.push(&design[end_idx..]);
            }
        }

        let num_ways = designs_to_check
            .iter()
            .map(|d| self.num_ways_to_make_design(d, cache))
            .sum();

        cache.insert(design.to_owned(), num_ways);

        num_ways
    }

    fn is_design_possible(&self, design: &str) -> bool {
        if design.is_empty() {
            return true;
        }

        let mut designs_to_check = Vec::new();

        for end_idx in 1..(design.len() + 1) {
            let towel = &design[0..end_idx];
            if self.towels.contains(towel) {
                designs_to_check.push(&design[end_idx..]);
            }
        }

        designs_to_check.iter().any(|d| self.is_design_possible(d))
    }
}
