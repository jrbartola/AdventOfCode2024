use common::coordinate::Coordinate;
use common::grid::{Grid, GridDirection};
use std::collections::HashSet;

pub enum GuardTickResult {
    Turned(Coordinate, GridDirection),
    Moved(Coordinate),
    NotInGrid,
}

pub struct GuardWalkResult {
    pub visited_cells: HashSet<Coordinate>,
    pub result_type: GuardWalkResultType,
}

#[derive(Debug, PartialEq)]
pub enum GuardWalkResultType {
    FellOffGrid,
    StuckInLoop,
}

const GUARD_UP: char = '^';
const GUARD_DOWN: char = 'v';
const GUARD_LEFT: char = '<';
const GUARD_RIGHT: char = '>';

pub struct GuardGrid {
    grid: Grid,
    pub guard_position: Coordinate,
    pub guard_direction: GridDirection,
    guard_is_on_grid: bool,
}

impl GuardGrid {
    pub fn from(lines: &Vec<String>) -> Self {
        let grid = Grid::from(&lines);
        let (guard_position, guard_direction) = Self::find_guard_pos_and_dir(&grid);

        Self {
            grid,
            guard_position,
            guard_direction,
            guard_is_on_grid: true,
        }
    }

    pub fn walk_guard(&mut self) -> GuardWalkResult {
        let mut turns: HashSet<(Coordinate, GridDirection)> = HashSet::new();
        let mut walked_positions: HashSet<Coordinate> = HashSet::new();
        walked_positions.insert(self.guard_position.clone());

        loop {
            let tick_result = self.tick();

            match tick_result {
                GuardTickResult::Moved(coords) => {
                    walked_positions.insert(coords);
                }
                GuardTickResult::NotInGrid => {
                    return GuardWalkResult {
                        visited_cells: walked_positions,
                        result_type: GuardWalkResultType::FellOffGrid,
                    }
                }
                GuardTickResult::Turned(coords, direction) => {
                    if turns.contains(&(coords.clone(), direction)) {
                        return GuardWalkResult {
                            visited_cells: walked_positions,
                            result_type: GuardWalkResultType::StuckInLoop,
                        };
                    } else {
                        turns.insert((coords, direction));
                    }
                }
            }
        }
    }

    pub fn place_obstacle(&mut self, coords: &Coordinate) {
        self.grid.set(coords, '#');
    }

    pub fn remove_obstacle(&mut self, coords: &Coordinate) {
        self.grid.set(coords, '.');
    }

    fn tick(&mut self) -> GuardTickResult {
        // 1. Check if guard is still on the grid
        if !self.guard_is_on_grid {
            return GuardTickResult::NotInGrid;
        }

        // 2. Get the next coordinates for the guard to move to
        let next_coords = match self.guard_direction {
            GridDirection::Up => {
                if self.guard_position.0 == 0 {
                    None
                } else {
                    Some(Coordinate(self.guard_position.0 - 1, self.guard_position.1))
                }
            }
            GridDirection::Down => {
                if self.guard_position.0 == self.grid.row_len() - 1 {
                    None
                } else {
                    Some(Coordinate(self.guard_position.0 + 1, self.guard_position.1))
                }
            }
            GridDirection::Left => {
                if self.guard_position.1 == 0 {
                    None
                } else {
                    Some(Coordinate(self.guard_position.0, self.guard_position.1 - 1))
                }
            }
            GridDirection::Right => {
                if self.guard_position.1 == self.grid.col_len() - 1 {
                    None
                } else {
                    Some(Coordinate(self.guard_position.0, self.guard_position.1 + 1))
                }
            }
        };

        if next_coords.is_none() || !self.grid.is_in_bounds(&next_coords.clone().unwrap()) {
            self.guard_is_on_grid = false;
            return GuardTickResult::NotInGrid;
        }

        let next_coords = next_coords.unwrap();

        // 3. Check if those coordinates are blocked. If they are, turn right
        let next_cell = self.grid.get(&next_coords).unwrap();

        if next_cell == '#' {
            self.guard_direction = match self.guard_direction {
                GridDirection::Up => GridDirection::Right,
                GridDirection::Down => GridDirection::Left,
                GridDirection::Left => GridDirection::Up,
                GridDirection::Right => GridDirection::Down,
            };

            GuardTickResult::Turned(self.guard_position.clone(), self.guard_direction)
        } else {
            self.guard_position = next_coords.clone();

            GuardTickResult::Moved(next_coords)
        }
    }

    pub fn reset(&mut self, guard_position: Coordinate, guard_direction: GridDirection) {
        // let (guard_position, guard_direction) = Self::find_guard_pos_and_dir(&self.grid);

        self.guard_position = guard_position;
        self.guard_direction = guard_direction;
        self.guard_is_on_grid = true;
    }

    fn find_guard_pos_and_dir(grid: &Grid) -> (Coordinate, GridDirection) {
        if let Some(coords) = grid.find(GUARD_UP).get(0) {
            (coords.clone(), GridDirection::Up)
        } else if let Some(coords) = grid.find(GUARD_DOWN).get(0) {
            (coords.clone(), GridDirection::Down)
        } else if let Some(coords) = grid.find(GUARD_LEFT).get(0) {
            (coords.clone(), GridDirection::Left)
        } else if let Some(coords) = grid.find(GUARD_RIGHT).get(0) {
            (coords.clone(), GridDirection::Right)
        } else {
            panic!("Guard not found in grid");
        }
    }
}
