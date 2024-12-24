use common::coordinate::Coordinate;
use common::grid::Grid;
use std::collections::{HashSet, VecDeque};

const MAX_HEIGHT: u8 = 9;

pub struct TrailMap {
    grid: Grid<u8>,
}

impl TrailMap {
    pub fn from(lines: &Vec<String>) -> Self {
        Self {
            grid: Grid::from_generic(lines, |c| c as u8 - b'0'),
        }
    }

    pub fn search_score(&self) -> u32 {
        let trailheads = self.grid.find(&0);

        trailheads.iter().fold(0, |acc, trailhead| {
            acc + self.bfs_from(trailhead).0.len() as u32
        })
    }

    pub fn search_rating(&self) -> u32 {
        let trailheads = self.grid.find(&0);

        trailheads.iter().fold(0, |acc, trailhead| {
            acc + self.bfs_from(trailhead).1.len() as u32
        })
    }

    fn bfs_from(&self, start: &Coordinate) -> (HashSet<Coordinate>, Vec<Vec<Coordinate>>) {
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut destinations: HashSet<Coordinate> = HashSet::new();
        let mut paths: Vec<Vec<Coordinate>> = Vec::new();
        let mut queue: VecDeque<(Coordinate, Vec<Coordinate>)> =
            VecDeque::from([(start.clone(), vec![])]);

        while !queue.is_empty() {
            let (coord, path) = queue.pop_front().unwrap();
            let curr_value = self.grid.get(&coord).unwrap();

            if *curr_value == MAX_HEIGHT {
                paths.push({
                    let mut new_path = path.clone();
                    new_path.push(coord);
                    new_path
                });
                destinations.insert(coord);
                continue;
            }

            for adjacent in self.grid.get_adjacent(&coord) {
                let next_value = self.grid.get(&adjacent).unwrap();
                if visited.contains(&adjacent)
                    || curr_value > next_value
                    || next_value - curr_value != 1
                {
                    continue;
                }

                queue.push_back((adjacent, {
                    let mut new_path = path.clone();
                    new_path.push(coord);
                    new_path
                }));
            }
        }

        (destinations, paths)
    }
}
