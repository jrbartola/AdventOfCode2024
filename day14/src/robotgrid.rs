use common::coordinate::Coordinate;
use common::grid::Grid;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

const NUM_ROWS: usize = 103;
// const NUM_ROWS: usize = 7;
const NUM_COLS: usize = 101;
// const NUM_COLS: usize = 11;

lazy_static! {
    static ref ROBOT_COORD_REGEX: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    velocity: (i32, i32),
}

pub struct RobotGrid {
    grid: Grid<HashSet<Robot>>,
}

impl RobotGrid {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut grid = Grid::<HashSet<Robot>>::empty(NUM_ROWS, NUM_COLS, HashSet::new());

        for line in lines {
            let captures = ROBOT_COORD_REGEX.captures(line).unwrap();
            let row = captures[2].parse::<usize>().unwrap();
            let col = captures[1].parse::<usize>().unwrap();
            let velocity_x = captures[3].parse::<i32>().unwrap();
            let velocity_y = captures[4].parse::<i32>().unwrap();

            let robot = Robot {
                velocity: (velocity_x, velocity_y),
            };

            grid.get_mut(&Coordinate(row, col)).unwrap().insert(robot);
        }

        Self { grid }
    }

    pub fn tick(&mut self) -> () {
        let mut moves = Vec::new();

        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let robots = self.grid.get(&Coordinate(row, col)).unwrap().clone();
                for robot in robots {
                    let (velocity_x, velocity_y) = robot.velocity;
                    let new_row = (row as i32 + velocity_y).rem_euclid(NUM_ROWS as i32) as usize;
                    let new_col = (col as i32 + velocity_x).rem_euclid(NUM_COLS as i32) as usize;

                    moves.push((robot, row, col, new_row, new_col));
                }
            }
        }

        let mut new_grid: Grid<HashSet<Robot>> =
            Grid::<HashSet<Robot>>::empty(NUM_ROWS, NUM_COLS, HashSet::new());

        // Apply moves
        for (robot, row, col, new_row, new_col) in moves {
            new_grid
                .get_mut(&Coordinate(new_row, new_col))
                .unwrap()
                .insert(robot);
        }

        self.grid = new_grid;
    }

    pub fn parse_quadrants(&self) -> (usize, usize, usize, usize) {
        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;

        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let robots = self.grid.get(&Coordinate(row, col)).unwrap();
                if robots.len() > 0 {
                    if row < NUM_ROWS / 2 && col < NUM_COLS / 2 {
                        top_left += robots.len();
                    } else if row < NUM_ROWS / 2 && col > NUM_COLS / 2 {
                        top_right += robots.len();
                    } else if row > NUM_ROWS / 2 && col < NUM_COLS / 2 {
                        bottom_left += robots.len();
                    } else if row > NUM_ROWS / 2 && col > NUM_COLS / 2 {
                        bottom_right += robots.len();
                    }
                }
            }
        }

        (top_left, top_right, bottom_left, bottom_right)
    }

    pub fn print(&self) -> () {
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let robots = self.grid.get(&Coordinate(row, col)).unwrap();
                if robots.len() > 0 {
                    print!("{}", robots.len());
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
