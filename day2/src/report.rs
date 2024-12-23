pub struct Report {
    values: Vec<u32>,
}

#[derive(Eq, PartialEq)]
enum Direction {
    None,
    Ascending,
    Descending,
}

impl Report {
    pub fn new(values: Vec<u32>) -> Report {
        Report { values }
    }

    pub fn is_valid_dampener(&self) -> bool {
        (0..self.values.len()).any(|skipped_index| self.is_valid(Some(skipped_index)))
    }

    pub fn is_valid(&self, skipped_index: Option<usize>) -> bool {
        let mut direction = Direction::None;

        let mut i: usize = 0;
        let mut j: usize = 1;

        while j < self.values.len() {
            if skipped_index.is_some() {
                if i == skipped_index.unwrap() {
                    i += 1;

                    if i == j {
                        j += 1;
                    }
                    continue;
                }

                if j == skipped_index.unwrap() {
                    j += 1;
                    continue;
                }
            }

            let first = self.values[i] as i32;
            let second = self.values[j] as i32;

            if direction == Direction::None {
                if first < second {
                    direction = Direction::Ascending;
                }

                if first > second {
                    direction = Direction::Descending;
                }
            } else if direction == Direction::Ascending && first > second {
                return false;
            } else if direction == Direction::Descending && first < second {
                return false;
            }

            if !((second - first).abs() >= 1 && ((second - first).abs() <= 3)) {
                return false;
            }

            i += 1;
            j += 1;
        }

        true
    }
}
