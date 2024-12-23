use crate::pagerule::RuleMap;
use std::cmp::Ordering;
use std::collections::HashSet;

pub struct Update {
    pages: Vec<u32>,
}

impl Update {
    pub fn from_str(input: String) -> Self {
        let pages = input
            .split(",")
            .map(|page| page.parse::<u32>().unwrap())
            .collect();

        Self { pages }
    }
    pub fn new(pages: Vec<u32>) -> Self {
        Self { pages }
    }

    pub fn is_valid_for_rulemap(&self, rule_map: &RuleMap) -> bool {
        let mut pages_seen = HashSet::new();

        for page in &self.pages {
            if let Some(after) = rule_map.get_after(page) {
                // If we've already seen a page that's supposed to come after the current one, then the update is invalid
                if after.iter().any(|&p| pages_seen.contains(p)) {
                    return false;
                }
            }

            pages_seen.insert(page);
        }

        true
    }

    pub fn reorder(&mut self, rule_map: &RuleMap) -> () {
        let empty_vec = vec![];

        self.pages.sort_by(|a, b| {
            let a_after = rule_map.get_after(a).unwrap_or(&empty_vec);
            let b_before = rule_map.get_before(b).unwrap_or(&empty_vec);

            if a_after.contains(&b) {
                Ordering::Less
            } else if b_before.contains(&a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }

    pub fn get_middle_page(&self) -> u32 {
        let middle_idx = (self.pages.len() - 1) / 2;
        self.pages[middle_idx]
    }
}
