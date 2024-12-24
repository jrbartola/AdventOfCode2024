use crate::coordinate::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Grid<T> {
    cells: Vec<Vec<T>>,
}

impl Grid<char> {
    pub fn from(lines: &Vec<String>) -> Grid<char> {
        Grid {
            cells: lines
                .into_iter()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
}

impl<T: PartialEq<T>> Grid<T> {
    pub fn from_generic(lines: &Vec<String>, collector: impl Fn(char) -> T) -> Grid<T> {
        Grid {
            cells: lines
                .into_iter()
                .map(|line| line.chars().map(|c| collector(c)).collect())
                .collect(),
        }
    }

    pub fn row_len(&self) -> usize {
        self.cells.len()
    }

    pub fn col_len(&self) -> usize {
        self.cells[0].len()
    }

    pub fn get(&self, coordinate: &Coordinate) -> Option<&T> {
        let Coordinate(row, col) = coordinate;

        if !self.is_in_bounds(coordinate) {
            return None;
        }

        Some(&self.cells[*row][*col])
    }

    pub fn set(&mut self, coordinate: &Coordinate, value: T) {
        let Coordinate(row, col) = coordinate;

        if !self.is_in_bounds(coordinate) {
            return;
        }

        self.cells[*row][*col] = value;
    }

    // Returns a vector of coordinates where the given item can be found at
    pub fn find(&self, item: &T) -> Vec<Coordinate> {
        let mut coordinates = Vec::new();

        for (row_idx, row) in self.cells.iter().enumerate() {
            for (col_idx, cell_value) in row.iter().enumerate() {
                if *cell_value == *item {
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

    pub fn itercells(&self) -> impl Iterator<Item = &T> {
        self.cells.iter().flat_map(|row| row.iter())
    }

    pub fn enumercells(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.cells.iter().enumerate().flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_idx, cell)| (Coordinate(row_idx, col_idx), cell))
        })
    }

    pub fn get_adjacent(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        let mut adjacents = Vec::new();

        if coordinate.0 > 0 {
            adjacents.push(Coordinate(coordinate.0 - 1, coordinate.1));
        }

        if coordinate.1 > 0 {
            adjacents.push(Coordinate(coordinate.0, coordinate.1 - 1));
        }

        if coordinate.0 < self.row_len() - 1 {
            adjacents.push(Coordinate(coordinate.0 + 1, coordinate.1));
        }

        if coordinate.1 < self.col_len() - 1 {
            adjacents.push(Coordinate(coordinate.0, coordinate.1 + 1));
        }

        adjacents
    }
}
