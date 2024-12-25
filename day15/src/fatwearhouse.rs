use common::coordinate::Coordinate;
use common::grid::Grid;
use std::collections::VecDeque;

const WALL: char = '#';
const BOX: char = 'O';
const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';
const ROBOT: char = '@';
const EMPTY: char = '.';

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum StepDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct FatWarehouse {
    grid: Grid<char>,
    steps: Vec<StepDirection>,
}

impl FatWarehouse {
    pub fn from(lines: &Vec<String>) -> Self {
        let grid_lines = lines
            .iter()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .flat_map(|char| match char {
                        WALL => vec![WALL, WALL],

                        BOX => vec![BOX_LEFT, BOX_RIGHT],
                        EMPTY => vec![EMPTY, EMPTY],
                        ROBOT => vec![ROBOT, EMPTY],
                        _ => panic!("Bad char when fattening grid: {}", char),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        let steps: Vec<char> = lines
            .iter()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .flat_map(|line| line.chars())
            .collect::<Vec<char>>();

        Self {
            grid: Grid::from(&grid_lines),
            steps: steps
                .iter()
                .map(|&c| match c {
                    '^' => StepDirection::Up,
                    'v' => StepDirection::Down,
                    '<' => StepDirection::Left,
                    '>' => StepDirection::Right,
                    _ => panic!("Invalid step direction: {}", c),
                })
                .collect::<Vec<StepDirection>>(),
        }
    }

    pub fn run(&mut self) -> () {
        let mut robot_coords = self.grid.find(&'@')[0];

        let steps = self.steps.clone();

        for step in steps {
            robot_coords = self.run_step(&robot_coords, step);
            // println!("Step: {:?}", step);
            // self.print();
        }
    }

    pub fn compute_gps(&self) -> u64 {
        self.grid
            .enumercells()
            .fold(0, |acc, (coords, cell_value)| {
                if cell_value == &BOX_LEFT {
                    acc + 100u64 * coords.0 as u64 + coords.1 as u64
                } else {
                    acc
                }
            })
    }

    pub fn print(&self) -> () {
        self.grid.print();
    }

    fn run_step(&mut self, robot_coords: &Coordinate, step: StepDirection) -> Coordinate {
        let new_coords = match step {
            StepDirection::Up => Coordinate(robot_coords.0 - 1, robot_coords.1),
            StepDirection::Down => Coordinate(robot_coords.0 + 1, robot_coords.1),
            StepDirection::Left => Coordinate(robot_coords.0, robot_coords.1 - 1),
            StepDirection::Right => Coordinate(robot_coords.0, robot_coords.1 + 1),
        };

        let new_cell = self.grid.get(&new_coords);

        match new_cell {
            Some(&WALL) => robot_coords.clone(),
            Some(&BOX_LEFT) | Some(&BOX_RIGHT) => {
                if self.push(&new_coords, step) {
                    self.grid.set(&new_coords, ROBOT);
                    self.grid.set(robot_coords, '.');
                    new_coords
                } else {
                    robot_coords.clone()
                }
            }
            Some(&EMPTY) => {
                self.grid.set(&new_coords, ROBOT);
                self.grid.set(robot_coords, '.');
                new_coords
            }
            _ => panic!("Invalid cell: {:?}", new_cell),
        }
    }

    fn push(&mut self, coords: &Coordinate, direction: StepDirection) -> bool {
        match direction {
            StepDirection::Up => {
                let mut coords_to_push = Vec::new();
                let mut coords_queue = VecDeque::from([coords.clone()]);

                while let Some(coords) = coords_queue.pop_front() {
                    let cell = self.grid.get(&coords).unwrap();
                    if cell == &BOX_LEFT {
                        let coords_above = Coordinate(coords.0 - 1, coords.1);
                        let coords_above_right = Coordinate(coords.0 - 1, coords.1 + 1);
                        let coords_right = Coordinate(coords.0, coords.1 + 1);

                        let cell_above = self.grid.get(&coords_above);
                        let cell_above_right = self.grid.get(&coords_above_right);

                        if cell_above != Some(&WALL) && cell_above_right != Some(&WALL) {
                            coords_to_push.push((coords, coords_above, BOX_LEFT));
                            coords_to_push.push((coords_right, coords_above_right, BOX_RIGHT));
                            coords_queue.push_back(coords_above);
                            coords_queue.push_back(coords_above_right);
                        } else {
                            return false;
                        }
                    } else if cell == &BOX_RIGHT {
                        let coords_above = Coordinate(coords.0 - 1, coords.1);
                        let coords_above_left = Coordinate(coords.0 - 1, coords.1 - 1);
                        let coords_left = Coordinate(coords.0, coords.1 - 1);

                        let cell_above = self.grid.get(&coords_above);
                        let cell_above_left = self.grid.get(&coords_above_left);

                        if cell_above != Some(&WALL) && cell_above_left != Some(&WALL) {
                            coords_to_push.push((coords, coords_above, BOX_RIGHT));
                            coords_to_push.push((coords_left, coords_above_left, BOX_LEFT));
                            coords_queue.push_back(coords_above);
                            coords_queue.push_back(coords_above_left);
                        } else {
                            return false;
                        }
                    } else if cell == &EMPTY {
                        continue;
                    } else if cell == &WALL {
                        return false;
                    } else {
                        panic!("Invalid cell: {:?}", cell);
                    }
                }

                for (from_coords, to_coords, char) in coords_to_push.iter().rev() {
                    self.grid.set(&to_coords, *char);
                    self.grid.set(&from_coords, EMPTY);
                }

                true
            }
            StepDirection::Down => {
                let mut coords_to_push = Vec::new();
                let mut coords_queue = VecDeque::from([coords.clone()]);

                while let Some(coords) = coords_queue.pop_front() {
                    let cell = self.grid.get(&coords).unwrap();
                    if cell == &BOX_LEFT {
                        let coords_below = Coordinate(coords.0 + 1, coords.1);
                        let coords_below_right = Coordinate(coords.0 + 1, coords.1 + 1);
                        let coords_right = Coordinate(coords.0, coords.1 + 1);

                        let cell_below = self.grid.get(&coords_below);
                        let cell_below_right = self.grid.get(&coords_below_right);

                        if cell_below != Some(&WALL) && cell_below_right != Some(&WALL) {
                            coords_to_push.push((coords, coords_below, BOX_LEFT));
                            coords_to_push.push((coords_right, coords_below_right, BOX_RIGHT));
                            coords_queue.push_back(coords_below);
                            coords_queue.push_back(coords_below_right);
                        } else {
                            return false;
                        }
                    } else if cell == &BOX_RIGHT {
                        let coords_below = Coordinate(coords.0 + 1, coords.1);
                        let coords_below_left = Coordinate(coords.0 + 1, coords.1 - 1);
                        let coords_left = Coordinate(coords.0, coords.1 - 1);

                        let cell_below = self.grid.get(&coords_below);
                        let cell_below_left = self.grid.get(&coords_below_left);

                        if cell_below != Some(&WALL) && cell_below_left != Some(&WALL) {
                            coords_to_push.push((coords, coords_below, BOX_RIGHT));
                            coords_to_push.push((coords_left, coords_below_left, BOX_LEFT));
                            coords_queue.push_back(coords_below);
                            coords_queue.push_back(coords_below_left);
                        } else {
                            return false;
                        }
                    } else if cell == &EMPTY {
                        continue;
                    } else if cell == &WALL {
                        return false;
                    } else {
                        panic!("Invalid cell: {:?}", cell);
                    }
                }

                for (from_coords, to_coords, char) in coords_to_push.iter().rev() {
                    self.grid.set(&to_coords, *char);
                    self.grid.set(&from_coords, EMPTY);
                }

                true
            }
            StepDirection::Left => {
                let row = coords.0;
                let start_col = coords.1 - 1;
                let mut end_col = start_col;

                let mut end_col_cell = self.grid.get(&Coordinate(row, end_col));

                while end_col_cell == Some(&BOX_LEFT) || end_col_cell == Some(&BOX_RIGHT) {
                    end_col -= 1;
                    end_col_cell = self.grid.get(&Coordinate(row, end_col));
                }

                if end_col_cell == Some(&WALL) {
                    false
                } else if end_col_cell == Some(&EMPTY) {
                    let mut char = BOX_LEFT;
                    for col in end_col..=start_col {
                        self.grid.set(&Coordinate(row, col), char);
                        char = if char == BOX_LEFT {
                            BOX_RIGHT
                        } else {
                            BOX_LEFT
                        };
                    }

                    true
                } else {
                    panic!("Should not encounter a box cell: {:?}", end_col_cell);
                }
            }
            StepDirection::Right => {
                let row = coords.0;
                let start_col = coords.1 + 1;
                let mut end_col = start_col;

                let mut end_col_cell = self.grid.get(&Coordinate(row, end_col));

                while end_col_cell == Some(&BOX_LEFT) || end_col_cell == Some(&BOX_RIGHT) {
                    end_col += 1;
                    end_col_cell = self.grid.get(&Coordinate(row, end_col));
                }

                if end_col_cell == Some(&WALL) {
                    false
                } else if end_col_cell == Some(&EMPTY) {
                    let mut char = BOX_LEFT;
                    for col in start_col..=end_col {
                        self.grid.set(&Coordinate(row, col), char);
                        char = if char == BOX_LEFT {
                            BOX_RIGHT
                        } else {
                            BOX_LEFT
                        };
                    }

                    true
                } else {
                    panic!("Should not encounter a box cell: {:?}", end_col_cell);
                }
            }
        }
    }
}
