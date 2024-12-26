use common::coordinate::Coordinate;
use common::grid::Grid;
use std::collections::{HashSet, VecDeque};

const ROW_LEN: usize = 71;
const COL_LEN: usize = 71;

pub struct ByteSpace {
    grid: Grid<char>,
    byte_coords: Vec<Coordinate>,
}

impl ByteSpace {
    pub fn from(lines: &Vec<String>) -> Self {
        let mut grid = Grid::<char>::empty(ROW_LEN, COL_LEN, '.');

        let byte_coords = lines
            .iter()
            .map(|coords| {
                let coords_vec = coords
                    .split(",")
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                Coordinate(coords_vec[1], coords_vec[0])
            })
            .collect::<Vec<Coordinate>>();

        ByteSpace { grid, byte_coords }
    }

    pub fn find_blocking_byte(&mut self) -> (usize, usize) {
        // We can skip the first 1024 hydrations
        self.hydrate_bytes(0, 1024);

        for idx in 1..self.byte_coords.len() {
            let shortest_path_len = self.bfs(idx - 1, idx);

            if shortest_path_len == usize::MAX {
                return (self.byte_coords[idx - 1].0, self.byte_coords[idx - 1].1);
            }
        }

        (0, 0)
    }

    pub fn bfs(&mut self, len_start: usize, len_cutoff: usize) -> usize {
        self.hydrate_bytes(len_start, len_cutoff);

        let start_coord = Coordinate(0, 0);
        let end_coord = Coordinate(ROW_LEN - 1, COL_LEN - 1);

        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([(start_coord, 0usize)]);

        while let Some((coord, path_len)) = queue.pop_front() {
            if visited.contains(&coord) {
                continue;
            }

            if coord == end_coord {
                return path_len;
            }

            visited.insert(coord.clone());

            for adj_coord in self.grid.get_adjacent(&coord) {
                let adj_char = self.grid.get(&adj_coord);
                if adj_char == Some(&'.') && !visited.contains(&adj_coord) {
                    queue.push_back((adj_coord, path_len + 1));
                }
            }
        }

        usize::MAX
    }

    fn hydrate_bytes(&mut self, len_start: usize, len_cutoff: usize) {
        for coord in &self.byte_coords[len_start..len_cutoff] {
            self.grid.set(coord, '#');
        }
    }
}
