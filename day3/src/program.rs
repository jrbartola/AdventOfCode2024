use crate::eval_mul;
use regex::Regex;

use lazy_static::lazy_static;

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref MUL_WITH_CONDITIONALS_REGEX: Regex =
        Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
}

pub struct Program {
    lines: Vec<String>,
}

impl Program {
    pub fn new(lines: Vec<String>) -> Self {
        Program { lines }
    }

    pub fn run(&self) -> u32 {
        self.lines.iter().fold(0, |acc, line| {
            acc + MUL_REGEX
                .captures_iter(line)
                .into_iter()
                .fold(0, |total, mul_caps| {
                    total
                        + eval_mul(
                            mul_caps.get(1).unwrap().as_str(),
                            mul_caps.get(2).unwrap().as_str(),
                        )
                })
        })
    }

    pub fn run_with_conditionals(&self) -> u32 {
        let mut mul_enabled = true;

        self.lines.iter().fold(0, |acc, line| {
            MUL_WITH_CONDITIONALS_REGEX
                .captures_iter(line)
                .into_iter()
                .fold(0, |total, caps| {
                    if let Some(mul_caps) = caps.get(1) {
                        if mul_enabled {
                            let mul_caps = MUL_REGEX
                                .captures_iter(mul_caps.as_str())
                                .into_iter()
                                .next()
                                .unwrap();

                            return total
                                + eval_mul(
                                    mul_caps.get(1).unwrap().as_str(),
                                    mul_caps.get(2).unwrap().as_str(),
                                );
                        }
                    } else if caps.get(2).is_some() {
                        mul_enabled = true;
                    } else if caps.get(3).is_some() {
                        mul_enabled = false;
                    }

                    total
                })
        })
    }
}
