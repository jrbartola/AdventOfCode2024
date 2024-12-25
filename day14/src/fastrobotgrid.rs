use common::coordinate::Coordinate;
use common::grid::Grid;
use regex::Regex;
use std::collections::HashSet;

const NUM_ROWS: usize = 103;
const NUM_COLS: usize = 101;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robot {
    position: Coordinate,
    velocity: (i32, i32),
}

pub struct FastRobotGrid {
    robots: HashSet<Robot>,
}

impl FastRobotGrid {
    pub fn from(lines: &Vec<String>) -> Self {
        let robot_regex: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        let mut robots = HashSet::new();

        for line in lines {
            let captures = robot_regex.captures(line).unwrap();
            let row = captures[2].parse::<usize>().unwrap();
            let col = captures[1].parse::<usize>().unwrap();
            let velocity_x = captures[3].parse::<i32>().unwrap();
            let velocity_y = captures[4].parse::<i32>().unwrap();

            let robot = Robot {
                position: Coordinate(row, col),
                velocity: (velocity_x, velocity_y),
            };

            robots.insert(robot);
        }

        Self { robots }
    }

    pub fn tick(&mut self, num_ticks: u64) -> (HashSet<Robot>, Grid<HashSet<Robot>>) {
        let mut new_robots = HashSet::new();

        for robot in self.robots.iter() {
            let (velocity_x, velocity_y) = robot.velocity;
            let new_row = (robot.position.0 as i64 + (num_ticks as i64 * velocity_y as i64))
                .rem_euclid(NUM_ROWS as i64) as usize;
            let new_col = (robot.position.1 as i64 + (num_ticks as i64 * velocity_x as i64))
                .rem_euclid(NUM_COLS as i64) as usize;

            new_robots.insert(Robot {
                position: Coordinate(new_row, new_col),
                velocity: (velocity_x, velocity_y),
            });
        }

        self.robots = new_robots.clone();
        let new_grid = self.create_grid();

        (new_robots, new_grid)
    }

    pub fn parse_quadrants(&self) -> (usize, usize, usize, usize) {
        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;

        let grid = self.create_grid();

        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let robots = grid.get(&Coordinate(row, col)).unwrap();
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
        let grid = self.create_grid();
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let robots = grid.get(&Coordinate(row, col)).unwrap();
                if robots.len() > 0 {
                    print!("{}", robots.len());
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn create_grid(&self) -> Grid<HashSet<Robot>> {
        let mut grid: Grid<HashSet<Robot>> =
            Grid::<HashSet<Robot>>::empty(NUM_ROWS, NUM_COLS, HashSet::new());

        for robot in self.robots.iter() {
            let robots = grid.get_mut(&robot.position).unwrap();
            robots.insert(*robot);
        }

        grid
    }
}
