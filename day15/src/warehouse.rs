use common::coordinate::Coordinate;
use common::grid::Grid;

const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';
const EMPTY: char = '.';

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum StepDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Warehouse {
    grid: Grid<char>,
    steps: Vec<StepDirection>,
}

impl Warehouse {
    pub fn from(lines: &Vec<String>) -> Self {
        let grid_lines = lines
            .iter()
            .take_while(|line| !line.is_empty())
            .map(|line| line.to_owned())
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
        }
    }

    pub fn compute_gps(&self) -> u64 {
        self.grid
            .enumercells()
            .fold(0, |acc, (coords, cell_value)| {
                if cell_value == &BOX {
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
            Some(&BOX) => {
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
                let col = coords.1;
                let start_row = coords.0 - 1;
                let mut end_row = start_row;

                while self.grid.get(&Coordinate(end_row, col)) == Some(&BOX) {
                    end_row -= 1;
                }

                let end_row_cell = self.grid.get(&Coordinate(end_row, col));
                if end_row_cell == Some(&WALL) {
                    false
                } else if end_row_cell == Some(&EMPTY) {
                    for row in end_row..=start_row {
                        self.grid.set(&Coordinate(row, col), BOX);
                    }

                    true
                } else {
                    panic!("Should not encounter a box cell: {:?}", end_row_cell);
                }
            }
            StepDirection::Down => {
                let col = coords.1;
                let start_row = coords.0 + 1;
                let mut end_row = start_row;

                while self.grid.get(&Coordinate(end_row, col)) == Some(&BOX) {
                    end_row += 1;
                }

                let end_row_cell = self.grid.get(&Coordinate(end_row, col));
                if end_row_cell == Some(&WALL) {
                    false
                } else if end_row_cell == Some(&EMPTY) {
                    for row in start_row..=end_row {
                        self.grid.set(&Coordinate(row, col), BOX);
                    }

                    true
                } else {
                    panic!("Should not encounter a box cell: {:?}", end_row_cell);
                }
            }
            StepDirection::Left => {
                let row = coords.0;
                let start_col = coords.1 - 1;
                let mut end_col = start_col;

                while self.grid.get(&Coordinate(row, end_col)) == Some(&BOX) {
                    end_col -= 1;
                }

                let end_col_cell = self.grid.get(&Coordinate(row, end_col));
                if end_col_cell == Some(&WALL) {
                    false
                } else if end_col_cell == Some(&EMPTY) {
                    for col in end_col..=start_col {
                        self.grid.set(&Coordinate(row, col), BOX);
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

                while self.grid.get(&Coordinate(row, end_col)) == Some(&BOX) {
                    end_col += 1;
                }

                let end_col_cell = self.grid.get(&Coordinate(row, end_col));
                if end_col_cell == Some(&WALL) {
                    false
                } else if end_col_cell == Some(&EMPTY) {
                    for col in start_col..=end_col {
                        self.grid.set(&Coordinate(row, col), BOX);
                    }

                    true
                } else {
                    panic!("Should not encounter a box cell: {:?}", end_col_cell);
                }
            }
        }
    }
}
