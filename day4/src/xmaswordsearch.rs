use crate::wordsearch::WordSearch;

pub struct XmasWordSearch {
    word_search: WordSearch,
}

impl XmasWordSearch {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            word_search: WordSearch::new(lines),
        }
    }

    pub fn solve(&self, word: &str) -> u32 {
        let bounding_dims = word.len();

        let mut solvable_cropped_ws = 0;

        for i in 0..(self.word_search.len() - (bounding_dims - 1)) {
            for j in 0..(self.word_search.len() - (bounding_dims - 1)) {
                let cropped_ws = self
                    .word_search
                    .crop((i, j), (i + bounding_dims - 1, j + bounding_dims - 1));

                if self.solve_cropped_ws(cropped_ws, word) {
                    solvable_cropped_ws += 1;
                }
            }
        }

        solvable_cropped_ws
    }

    fn solve_cropped_ws(&self, cropped_ws: WordSearch, word: &str) -> bool {
        let diagonals = cropped_ws.get_diagonals();

        diagonals.iter().fold(0, |acc, line| {
            acc + WordSearch::check_line_for_word(line, word)
        }) == 2
    }
}
