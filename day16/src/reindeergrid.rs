use crate::reindeergrid::GridCell::{End, Start};
use common::coordinate::Coordinate;
use common::grid::{Grid, GridDirection};
use std::collections::HashSet;
use std::fmt::Formatter;
use std::{cmp, fmt};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GridCell {
    Start,
    End,
    Empty,
    Wall,

    Up,
    Down,
    Left,
    Right,
}

enum TurnDirection {
    Clockwise,
    CounterClockwise,
}

impl fmt::Display for GridCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let char_cell = match self {
            GridCell::Start => 'S',
            GridCell::End => 'E',
            GridCell::Empty => '.',
            GridCell::Wall => '#',
            GridCell::Up => '^',
            GridCell::Down => 'v',
            GridCell::Left => '<',
            GridCell::Right => '>',
        };

        write!(f, "{}", char_cell)
    }
}

pub struct ReindeerGrid {
    grid: Grid<GridCell>,
}

impl ReindeerGrid {
    pub fn from(lines: &Vec<String>) -> Self {
        Self {
            grid: Grid::from_generic(lines, |c| match c {
                'S' => GridCell::Start,
                'E' => GridCell::End,
                '.' => GridCell::Empty,
                '#' => GridCell::Wall,
                _ => panic!("Invalid character in grid: {}", c),
            }),
        }
    }

    pub fn get_lowest_score(&self) -> u64 {
        let start_coords = self.get_start();
        let end_coords = self.get_end();

        self.dfs(
            start_coords,
            GridDirection::Right,
            HashSet::new(),
            0,
            &end_coords,
            Vec::new(),
        )
    }

    fn dfs(
        &self,
        next_coords: Coordinate,
        direction: GridDirection,
        visited: HashSet<(Coordinate, GridDirection)>,
        score: u64,
        end_coords: &Coordinate,
        path: Vec<(Coordinate, GridDirection)>,
    ) -> u64 {
        // self.print(&path);
        // println!("visited: {:?}", visited.len());
        if visited.contains(&(next_coords, direction)) {
            return u64::MAX;
        }

        if next_coords == *end_coords {
            return score;
        }

        let mut curr_min_score = score;

        let mut new_visited = visited.clone();
        new_visited.insert((next_coords, direction));

        for adj_coords in self.grid.get_adjacent(&next_coords) {
            if new_visited.contains(&(adj_coords, direction))
                || self.grid.get(&adj_coords) == Some(&GridCell::Wall)
            {
                continue;
            }

            if Self::is_going_same_direction(&next_coords, &adj_coords, direction) {
                let new_path = [path.clone(), vec![(next_coords, direction)]].concat();

                curr_min_score = cmp::min(
                    curr_min_score,
                    self.dfs(
                        adj_coords,
                        direction,
                        new_visited.clone(),
                        score.saturating_add(1),
                        end_coords,
                        new_path,
                    ),
                );
            } else {
                let (num_turns, turn_direction) =
                    Self::get_turns_to_face_dir(&next_coords, &adj_coords, direction);

                let new_direction = match turn_direction {
                    TurnDirection::Clockwise => {
                        if num_turns == 1 {
                            direction.turn_clockwise()
                        } else {
                            direction.turn_clockwise().turn_clockwise()
                        }
                    }
                    TurnDirection::CounterClockwise => {
                        if num_turns == 1 {
                            direction.turn_counter_clockwise()
                        } else {
                            direction.turn_counter_clockwise().turn_counter_clockwise()
                        }
                    }
                };

                if new_visited.contains(&(adj_coords, new_direction)) {
                    continue;
                }

                let new_path = [path.clone(), vec![(next_coords, new_direction)]].concat();

                curr_min_score = cmp::min(
                    curr_min_score,
                    self.dfs(
                        adj_coords.clone(),
                        new_direction,
                        new_visited.clone(),
                        score.saturating_add(num_turns as u64 * 1000 + 1),
                        end_coords,
                        new_path.clone(),
                    ),
                );
            }
        }

        curr_min_score
    }

    fn print(&self, path: &Vec<(Coordinate, GridDirection)>) {
        let mut grid = self.grid.clone();

        for (coords, direction) in path {
            grid.set(
                coords,
                match direction {
                    GridDirection::Up => GridCell::Up,
                    GridDirection::Down => GridCell::Down,
                    GridDirection::Left => GridCell::Left,
                    GridDirection::Right => GridCell::Right,
                },
            );
        }

        // Print the grid manually
        for row in 0..grid.row_len() {
            for col in 0..grid.col_len() {
                let cell = grid.get(&Coordinate(row, col)).unwrap();
                print!("{}", cell);
            }
            println!();
        }
    }

    fn get_start(&self) -> Coordinate {
        self.grid.find(&Start)[0]
    }

    fn get_end(&self) -> Coordinate {
        self.grid.find(&End)[0]
    }

    fn is_going_same_direction(
        start_coords: &Coordinate,
        adj_coords: &Coordinate,
        direction: GridDirection,
    ) -> bool {
        match direction {
            GridDirection::Up => adj_coords.0 < start_coords.0,
            GridDirection::Down => adj_coords.0 > start_coords.0,
            GridDirection::Left => adj_coords.1 < start_coords.1,
            GridDirection::Right => adj_coords.1 > start_coords.1,
        }
    }

    fn get_turns_to_face_dir(
        start_coords: &Coordinate,
        adj_coords: &Coordinate,
        direction: GridDirection,
    ) -> (usize, TurnDirection) {
        match direction {
            GridDirection::Up => {
                if adj_coords.1 < start_coords.1 {
                    (1, TurnDirection::CounterClockwise)
                } else if adj_coords.1 > start_coords.1 {
                    (1, TurnDirection::Clockwise)
                } else {
                    (2, TurnDirection::Clockwise)
                }
            }
            GridDirection::Down => {
                if adj_coords.1 > start_coords.1 {
                    (1, TurnDirection::CounterClockwise)
                } else if adj_coords.1 < start_coords.1 {
                    (1, TurnDirection::Clockwise)
                } else {
                    (2, TurnDirection::Clockwise)
                }
            }
            GridDirection::Left => {
                if adj_coords.0 < start_coords.0 {
                    (1, TurnDirection::Clockwise)
                } else if adj_coords.0 > start_coords.0 {
                    (1, TurnDirection::CounterClockwise)
                } else {
                    (2, TurnDirection::Clockwise)
                }
            }
            GridDirection::Right => {
                if adj_coords.0 > start_coords.0 {
                    (1, TurnDirection::Clockwise)
                } else if adj_coords.0 < start_coords.0 {
                    (1, TurnDirection::CounterClockwise)
                } else {
                    (2, TurnDirection::Clockwise)
                }
            }
        }
    }
}
