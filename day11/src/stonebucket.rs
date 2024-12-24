use std::collections::HashMap;

pub struct StoneBucket {
    initial_stones: Vec<u64>,
    stone_freqs: HashMap<u64, u64>,
}

const MULTIPLIER: u64 = 2024;

impl StoneBucket {
    pub fn from(line: String) -> Self {
        let stones: Vec<u64> = line
            .split_whitespace()
            .map(|stone| stone.parse::<u64>().unwrap())
            .collect();

        Self {
            stone_freqs: stones.iter().map(|&stone| (stone, 1)).collect(),
            initial_stones: stones,
        }
    }

    pub fn blink(&mut self) -> () {
        let num_zeroes = *self.stone_freqs.get(&0).unwrap_or(&0);
        let mut additions = vec![(1, num_zeroes)];
        let mut removals = vec![(0, num_zeroes)];

        for (&stone, &num_stones) in self.stone_freqs.iter() {
            if stone == 0 || num_stones == 0 {
                continue;
            }

            let num_digits = Self::digit_count(stone);

            if num_digits % 2 == 0 {
                let (left, right) = Self::split(stone, num_digits);
                additions.push((left, num_stones));
                additions.push((right, num_stones));
                removals.push((stone, num_stones));
            } else {
                additions.push((stone * MULTIPLIER, num_stones));
                removals.push((stone, num_stones));
            }
        }

        for (stone, num_stones) in additions {
            *self.stone_freqs.entry(stone).or_insert(0) += num_stones;
        }

        for (stone, num_stones) in removals {
            *self.stone_freqs.entry(stone).or_insert(0) -= num_stones;
        }
    }

    pub fn len(&self) -> u64 {
        self.stone_freqs.values().sum::<u64>()
    }

    fn split(stone: u64, num_digits: usize) -> (u64, u64) {
        let half = num_digits / 2;
        let divisor = 10_u64.pow(half as u32);
        let left = stone / divisor;
        let right = stone % divisor;
        (left, right)
    }

    fn digit_count(mut n: u64) -> usize {
        if n == 0 {
            return 1;
        }
        let mut count = 0;
        while n > 0 {
            n /= 10;
            count += 1;
        }
        count
    }
}
