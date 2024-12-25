use crate::flowergrid::SideOrientation::{Horizontal, Vertical};
use common::coordinate::Coordinate;
use common::grid::{Grid, GridDirection};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct FlowerGrid {
    grid: Grid<char>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum SideOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct SidePiece {
    orientation: SideOrientation,
    row: usize,
    col: usize,
    facing: GridDirection,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Side {
    Vertical {
        col: usize,
        row_start: usize,
        row_end: usize,
        facing: GridDirection,
    },
    Horizontal {
        row: usize,
        col_start: usize,
        col_end: usize,
        facing: GridDirection,
    },
}

impl FlowerGrid {
    pub fn from(lines: &Vec<String>) -> Self {
        Self {
            grid: Grid::from(lines),
        }
    }

    pub fn get_price(&self) -> u32 {
        // 1. First count each region, mapping each coordinate to a region
        let coords_to_region = self.bfs();

        // 2. Then, for each region, find the coordinates it has
        let regions_to_coords =
            coords_to_region
                .iter()
                .fold(HashMap::new(), |mut acc, (coords, region_id)| {
                    acc.entry(region_id)
                        .or_insert(HashSet::new())
                        .insert(*coords);
                    acc
                });

        // 3. Last, for each region, get its area and perimeter
        let perimeters_for_regions = regions_to_coords
            .iter()
            .map(|(&region_id, region)| (*region_id, self.calc_perimeter_for_region(region)))
            .collect::<HashMap<u32, u32>>();

        // 4. Return the sum of all (area * perimeter) for all regions
        perimeters_for_regions
            .iter()
            .fold(0, |acc, (region_id, perimeter)| {
                let area = regions_to_coords.get(region_id).unwrap().len();
                acc + (area as u32) * perimeter
            })
    }

    pub fn get_discouted_price(&self) -> u32 {
        // 1. First count each region, mapping each coordinate to a region
        let coords_to_region = self.bfs();

        // 2. Then, for each region, find the coordinates it has
        let regions_to_coords =
            coords_to_region
                .iter()
                .fold(HashMap::new(), |mut acc, (coords, region_id)| {
                    acc.entry(region_id)
                        .or_insert(HashSet::new())
                        .insert(*coords);
                    acc
                });

        // 3. For each region, get its side pieces
        let sides_for_regions = regions_to_coords
            .iter()
            .map(|(&region_id, region)| (*region_id, self.calc_sides_for_region(region)))
            .collect::<HashMap<u32, HashSet<SidePiece>>>();

        // 4. Piece the side pieces together to get the sides
        let full_sides = sides_for_regions
            .iter()
            .map(|(&region_id, side_pieces)| (region_id, self.combine_to_sides(side_pieces)))
            .collect::<HashMap<u32, HashSet<Side>>>();

        // 4. Return the sum of all (area * sides) for all regions
        full_sides
            .iter()
            .map(|(region_id, sides)| {
                let area = regions_to_coords.get(region_id).unwrap().len();
                area as u32 * (sides.len() as u32)
            })
            .sum()
    }

    fn bfs(&self) -> HashMap<Coordinate, u32> {
        let mut region_id = 0u32;
        let mut coords_to_region = HashMap::new();

        for i in 0..self.grid.row_len() {
            for j in 0..self.grid.col_len() {
                let coords = Coordinate(i, j);

                if coords_to_region.contains_key(&coords) {
                    continue;
                }

                // let region_id = *self.grid.get(&coords).unwrap();
                let region = self.region_bfs(coords);
                region.iter().for_each(|coord| {
                    coords_to_region.insert(*coord, region_id);
                });

                region_id += 1;
            }
        }

        coords_to_region
    }

    fn region_bfs(&self, start_coords: Coordinate) -> HashSet<Coordinate> {
        let mut region = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([start_coords]);

        while !queue.is_empty() {
            let coords = queue.pop_front().unwrap();

            if visited.contains(&coords) {
                continue;
            }

            region.insert(coords);
            visited.insert(coords);

            let adjacent = self.grid.get_adjacent(&coords);

            for adjacent_coords in adjacent {
                if !visited.contains(&adjacent_coords)
                    && *self.grid.get(&adjacent_coords).unwrap()
                        == *self.grid.get(&start_coords).unwrap()
                {
                    queue.push_back(adjacent_coords);
                }
            }
        }

        region
    }

    fn calc_perimeter_for_region(&self, region: &HashSet<Coordinate>) -> u32 {
        let grid = &self.grid;
        let mut perimeter = 0;

        let calc_perimeter = |coords: &Coordinate| -> u32 {
            let adjacent = grid.get_adjacent(coords);
            let mut perimeter = 0;

            if coords.0 == 0 {
                perimeter += 1;
            }
            if coords.0 == grid.row_len() - 1 {
                perimeter += 1;
            }
            if coords.1 == 0 {
                perimeter += 1;
            }
            if coords.1 == grid.col_len() - 1 {
                perimeter += 1;
            }

            for adjacent_coords in adjacent {
                if !region.contains(&adjacent_coords) {
                    perimeter += 1;
                }
            }

            perimeter
        };

        for coords in region {
            perimeter += calc_perimeter(&coords);
        }

        perimeter
    }

    fn calc_sides_for_region(&self, region: &HashSet<Coordinate>) -> HashSet<SidePiece> {
        let mut side_pieces = HashSet::new();

        for coords in region {
            let adjacent = self.grid.get_adjacent(coords);

            if coords.0 == 0 {
                side_pieces.insert(SidePiece {
                    orientation: Horizontal,
                    row: 0,
                    col: coords.1,
                    facing: GridDirection::Up,
                });
            }

            if coords.0 == self.grid.row_len() - 1 {
                side_pieces.insert(SidePiece {
                    orientation: Horizontal,
                    row: self.grid.row_len(),
                    col: coords.1,
                    facing: GridDirection::Down,
                });
            }

            if coords.1 == 0 {
                side_pieces.insert(SidePiece {
                    orientation: Vertical,
                    row: coords.0,
                    col: 0,
                    facing: GridDirection::Left,
                });
            }

            if coords.1 == self.grid.col_len() - 1 {
                side_pieces.insert(SidePiece {
                    orientation: Vertical,
                    row: coords.0,
                    col: self.grid.col_len(),
                    facing: GridDirection::Right,
                });
            }

            for adj_coords in adjacent {
                if !region.contains(&adj_coords) {
                    let orientation = if adj_coords.0 == coords.0 {
                        Vertical
                    } else {
                        Horizontal
                    };

                    let sp = SidePiece {
                        row: if orientation == Vertical {
                            adj_coords.0
                        } else {
                            std::cmp::max(coords.0, adj_coords.0)
                        },
                        col: if orientation == Vertical {
                            std::cmp::max(coords.1, adj_coords.1)
                        } else {
                            adj_coords.1
                        },
                        facing: if orientation == Vertical {
                            if adj_coords.1 < coords.1 {
                                GridDirection::Left
                            } else {
                                GridDirection::Right
                            }
                        } else {
                            if adj_coords.0 < coords.0 {
                                GridDirection::Up
                            } else {
                                GridDirection::Down
                            }
                        },
                        orientation,
                    };

                    side_pieces.insert(sp);
                }
            }
        }

        side_pieces
    }

    fn combine_to_sides(&self, side_pieces: &HashSet<SidePiece>) -> HashSet<Side> {
        let mut sides = HashSet::new();

        // 1. Group side pieces by orientation and major axis
        let mut verticals = HashMap::new();
        let mut horizontals = HashMap::new();

        for side_piece in side_pieces {
            match side_piece.orientation {
                Vertical => {
                    verticals
                        .entry(side_piece.col)
                        .or_insert(Vec::new())
                        .push((side_piece.row, side_piece.facing));
                }
                Horizontal => {
                    horizontals
                        .entry(side_piece.row)
                        .or_insert(Vec::new())
                        .push((side_piece.col, side_piece.facing));
                }
            }
        }

        // 2. For each orientation, sort the values and combine them into sides
        for (col, rows) in verticals {
            let mut sorted_rows = rows.iter().collect::<Vec<_>>();
            sorted_rows.sort_by(|(row1, _), (row2, _)| row1.partial_cmp(row2).unwrap());

            let mut row_start = sorted_rows[0].0;
            let mut row_direction = sorted_rows[0].1;
            let mut row_end = row_start;

            for i in 1..sorted_rows.len() {
                // If the next row is not adjacent to the current row, or the facing is the wrong direction

                if row_end + 1 != sorted_rows[i].0 || sorted_rows[i].1 != row_direction {
                    sides.insert(Side::Vertical {
                        col,
                        row_start,
                        row_end,
                        facing: row_direction,
                    });

                    row_start = sorted_rows[i].0;
                    row_end = sorted_rows[i].0;
                    row_direction = sorted_rows[i].1;
                } else {
                    row_end = sorted_rows[i].0;
                }
            }

            sides.insert(Side::Vertical {
                col,
                row_start,
                row_end,
                facing: row_direction,
            });
        }

        for (row, cols) in horizontals {
            let mut sorted_cols = cols.iter().collect::<Vec<_>>();
            sorted_cols.sort_by(|(col1, _), (col2, _)| col1.partial_cmp(col2).unwrap());

            let mut col_start = sorted_cols[0].0;
            let mut col_direction = sorted_cols[0].1;
            let mut col_end = col_start;

            for i in 1..sorted_cols.len() {
                if col_end + 1 != sorted_cols[i].0 || sorted_cols[i].1 != col_direction {
                    sides.insert(Side::Horizontal {
                        row,
                        col_start,
                        col_end,
                        facing: col_direction,
                    });

                    col_start = sorted_cols[i].0;
                    col_end = sorted_cols[i].0;
                    col_direction = sorted_cols[i].1;
                } else {
                    col_end = sorted_cols[i].0;
                }
            }

            sides.insert(Side::Horizontal {
                row,
                col_start,
                col_end,
                facing: col_direction,
            });
        }

        sides
    }
}
