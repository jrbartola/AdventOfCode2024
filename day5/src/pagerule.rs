use std::collections::HashMap;

pub struct PageRule {
    pub before_page: u32,
    pub after_page: u32,
}

impl PageRule {
    pub fn from_str(input: String) -> Self {
        let pages: Vec<u32> = input
            .split("|")
            .map(|page| page.parse::<u32>().unwrap())
            .collect();

        Self {
            before_page: pages[0],
            after_page: pages[1],
        }
    }

    pub fn new(before_page: u32, after_page: u32) -> Self {
        Self {
            before_page,
            after_page,
        }
    }
}

struct RuleMapValue<'a> {
    after: Vec<&'a u32>,
    before: Vec<&'a u32>,
}

pub struct RuleMap<'a> {
    rules: Vec<&'a PageRule>,
    map: HashMap<u32, RuleMapValue<'a>>,
}

impl<'a> RuleMap<'a> {
    pub fn from(rules: Vec<&'a PageRule>) -> Self {
        Self {
            map: RuleMap::make_map(&rules),
            rules,
        }
    }

    pub fn get_before(&self, page: &u32) -> Option<&Vec<&u32>> {
        self.map.get(&page).map(|value| &value.before)
    }

    pub fn get_after(&self, page: &u32) -> Option<&Vec<&u32>> {
        self.map.get(&page).map(|value| &value.after)
    }

    fn make_map(rules: &Vec<&'a PageRule>) -> HashMap<u32, RuleMapValue<'a>> {
        let mut map = HashMap::new();
        for rule in rules {
            let before = map.entry(rule.before_page).or_insert(RuleMapValue {
                before: Vec::new(),
                after: Vec::new(),
            });
            before.after.push(&rule.after_page);

            let after = map.entry(rule.after_page).or_insert(RuleMapValue {
                before: Vec::new(),
                after: Vec::new(),
            });
            after.before.push(&rule.before_page);
        }

        map
    }
}
