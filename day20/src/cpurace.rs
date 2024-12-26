use common::coordinate::Coordinate;
use common::grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone)]
pub struct CpuRace {
    pub grid: Grid<char>,
}

impl CpuRace {
    pub fn from(lines: &Vec<String>) -> Self {
        Self {
            grid: Grid::from(lines),
        }
    }

    pub fn is_cheat_effective(&self, first_coord: Coordinate, second_coord: Coordinate) -> bool {
        let orig_first_cell = *self.grid.get(&first_coord).unwrap();
        let orig_second_cell = *self.grid.get(&second_coord).unwrap();

        orig_second_cell == '#' || orig_first_cell == '#'
    }

    pub fn with_cheat(
        &mut self,
        first_coord: Coordinate,
        second_coord: Coordinate,
    ) -> (char, char) {
        let orig_first_cell = *self.grid.get(&first_coord).unwrap();
        let orig_second_cell = *self.grid.get(&second_coord).unwrap();

        if orig_first_cell != 'S' && orig_first_cell != 'E' {
            self.grid.set(&first_coord, '.');
        }

        if orig_second_cell != 'S' && orig_second_cell != 'E' {
            self.grid.set(&second_coord, '.');
        }

        (orig_first_cell, orig_second_cell)
    }

    pub fn remove_cheat(
        &mut self,
        first_coord: Coordinate,
        first_value: char,
        second_coord: Coordinate,
        second_value: char,
    ) -> () {
        self.grid.set(&first_coord, first_value);
        self.grid.set(&second_coord, second_value);
    }

    pub fn bfs(&self) -> u32 {
        let start_coord = self.get_start();

        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([(start_coord, 0u32)]);

        while let Some((coord, dist)) = queue.pop_front() {
            if visited.contains(&coord) {
                continue;
            }

            visited.insert(coord);

            let cell = self.grid.get(&coord);
            if cell == Some(&'E') {
                return dist;
            }

            for adj_coords in self.grid.get_adjacent(&coord) {
                let adj_cell = self.grid.get(&adj_coords);
                if visited.contains(&adj_coords) || adj_cell == Some(&'#') {
                    continue;
                }

                queue.push_back((adj_coords, dist + 1));
            }
        }

        u32::MAX
    }

    pub fn bfs_distances(&self) -> HashMap<Coordinate, u32> {
        let start_coord = self.get_start();

        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([(start_coord, 0u32)]);

        while let Some((coord, dist)) = queue.pop_front() {
            if visited.contains(&coord) {
                continue;
            }

            visited.insert(coord);
            distances.insert(coord, dist);

            let cell = self.grid.get(&coord);
            if cell == Some(&'E') {
                return distances;
            }

            for adj_coords in self.grid.get_adjacent(&coord) {
                let adj_cell = self.grid.get(&adj_coords);
                if visited.contains(&adj_coords) || adj_cell == Some(&'#') {
                    continue;
                }

                queue.push_back((adj_coords, dist + 1));
            }
        }

        HashMap::new()
    }

    pub fn apply_cheats(&self, dists: HashMap<Coordinate, u32>) -> HashMap<u32, u32> {
        let mut cheats_for_score = HashMap::new();
        let dirs_for_cheats = self.get_dirs_for_cheats(2, 20);

        for coords in dists.keys() {
            for dirs in dirs_for_cheats.iter() {
                let new_r = coords.0 as isize + dirs.0;
                let new_c = coords.1 as isize + dirs.1;

                // Ensure the new coordinates are within grid bounds
                if new_r < 0
                    || new_r >= self.grid.row_len() as isize
                    || new_c < 0
                    || new_c >= self.grid.col_len() as isize
                {
                    continue;
                }

                let new_coords = Coordinate(new_r as usize, new_c as usize);
                if !dists.contains_key(&new_coords) {
                    continue;
                }

                let new_dist = *dists.get(&new_coords).unwrap();
                let curr_dist = *dists.get(coords).unwrap();

                let dirs_manhattan = dirs.0.abs() + dirs.1.abs();

                // Calculate the cheat score and update the map
                let cheat_score = new_dist as isize - curr_dist as isize - dirs_manhattan;
                if cheat_score >= 100 {
                    *cheats_for_score.entry(cheat_score as u32).or_insert(0) += 1;
                }
            }
        }

        cheats_for_score
    }

    fn get_dirs_for_cheats(&self, min: isize, max: isize) -> Vec<(isize, isize)> {
        let mut dirs = Vec::new();

        for r in (-max)..(max + 1) {
            for c in (-max)..(max + 1) {
                if r == 0 && c == 0 {
                    continue;
                }

                if r.abs() + c.abs() >= min && r.abs() + c.abs() <= max {
                    dirs.push((r, c));
                }
            }
        }

        dirs
    }

    pub fn print_path(&self, path: &Vec<Coordinate>) -> () {
        let mut grid = self.grid.clone();

        for coord in path {
            grid.set(coord, 'O');
        }

        grid.print();
    }

    fn get_start(&self) -> Coordinate {
        self.grid.find(&'S')[0]
    }

    pub fn get_end(&self) -> Coordinate {
        self.grid.find(&'E')[0]
    }
}
