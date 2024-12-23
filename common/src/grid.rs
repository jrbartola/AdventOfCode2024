use crate::coordinate::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    pub fn from(lines: &Vec<String>) -> Self {
        Self {
            cells: lines
                .into_iter()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }

    pub fn row_len(&self) -> usize {
        self.cells.len()
    }

    pub fn col_len(&self) -> usize {
        self.cells[0].len()
    }

    pub fn get(&self, coordinate: &Coordinate) -> Option<char> {
        let Coordinate(row, col) = coordinate;

        if !self.is_in_bounds(coordinate) {
            return None;
        }

        Some(self.cells[*row][*col])
    }

    pub fn set(&mut self, coordinate: &Coordinate, value: char) {
        let Coordinate(row, col) = coordinate;

        if !self.is_in_bounds(coordinate) {
            return;
        }

        self.cells[*row][*col] = value;
    }

    // Returns a vector of coordinates where the given character can be found at
    pub fn find(&self, character: char) -> Vec<Coordinate> {
        let mut coordinates = Vec::new();

        for (row_idx, row) in self.cells.iter().enumerate() {
            for (col_idx, &cell_value) in row.iter().enumerate() {
                if cell_value == character {
                    coordinates.push(Coordinate(row_idx, col_idx))
                }
            }
        }

        coordinates
    }

    pub fn is_in_bounds(&self, coordinate: &Coordinate) -> bool {
        let Coordinate(row, col) = coordinate;

        *row < self.cells.len() && *col < self.cells[0].len()
    }
}
