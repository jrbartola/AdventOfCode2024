pub struct WordSearch {
    grid: Vec<Vec<char>>,
}

impl WordSearch {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            grid: lines
                .into_iter()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }

    pub fn crop(&self, (r1, c1): (usize, usize), (r2, c2): (usize, usize)) -> Self {
        Self {
            grid: self
                .grid
                .iter()
                .skip(r1)
                .take(r2 - r1 + 1)
                .map(|line| line.iter().skip(c1).take(c2 - c1 + 1).copied().collect())
                .collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    // Returns the total number of occurrences of the word in the grid
    pub fn solve(&self, word: &str) -> u32 {
        let horizontal = self.check_horizontals(word);
        let vertical = self.check_verticals(word);
        let diagonal = self.check_diagonals(word);

        horizontal + vertical + diagonal
    }

    pub fn get_horizontals(&self) -> Vec<String> {
        self.grid.iter().map(|line| line.iter().collect()).collect()
    }

    fn check_horizontals(&self, word: &str) -> u32 {
        self.get_horizontals().iter().fold(0, |acc, line| {
            acc + WordSearch::check_line_for_word(line, word)
        })
    }

    pub fn get_verticals(&self) -> Vec<String> {
        (0..self.grid[0].len())
            .into_iter()
            .map(|i| self.grid.iter().map(|line| line[i]).collect())
            .collect()
    }

    fn check_verticals(&self, word: &str) -> u32 {
        self.get_verticals().iter().fold(0, |acc, line| {
            acc + WordSearch::check_line_for_word(line, word)
        })
    }

    pub fn get_diagonals(&self) -> Vec<String> {
        // First, get forward diagonals (top-left to bottom-right)
        let forward_diags: Vec<String> = (0..2 * self.grid.len())
            .into_iter()
            .map(|i| {
                (0..=i)
                    .map(|j| {
                        if self.grid.len() > j && (i - j) < self.grid[0].len() {
                            Some(self.grid[self.grid.len() - j - 1][i - j])
                        } else {
                            None
                        }
                    })
                    .filter_map(|c| c)
                    .collect()
            })
            .filter(|diag: &String| diag.len() >= 1)
            .collect();

        // Next, get backward diagonals (top-right to bottom-left)
        let back_diags: Vec<String> = (0..2 * self.grid.len())
            .into_iter()
            .map(|i| {
                (0..=i)
                    .rev()
                    .map(|j| {
                        if (i - j) < self.grid.len() && j < self.grid[0].len() {
                            Some(self.grid[i - j][j])
                        } else {
                            None
                        }
                    })
                    .filter_map(|c| c)
                    .collect()
            })
            .filter(|diag: &String| diag.len() >= 1)
            .collect();

        forward_diags
            .into_iter()
            .chain(back_diags.into_iter())
            .collect()
    }

    fn check_diagonals(&self, word: &str) -> u32 {
        self.get_diagonals().iter().fold(0, |acc, line| {
            acc + WordSearch::check_line_for_word(line, word)
        })
    }

    pub(crate) fn check_line_for_word(line: &str, word: &str) -> u32 {
        let reverse_word: String = word.chars().rev().collect();

        (line.matches(word).count() + line.matches(&reverse_word).count()) as u32
    }
}
