use common::coordinate::Coordinate;
use common::grid::Grid;
use std::collections::{HashMap, HashSet};

pub struct AntennaGrid {
    grid: Grid<HashSet<char>>,
}

impl AntennaGrid {
    pub fn new(lines: &Vec<String>) -> Self {
        Self {
            grid: Grid::from_generic(lines, |c| HashSet::from([c])),
        }
    }

    pub fn generate_antinodes(&mut self, nodes_in_direction: usize) -> HashSet<Coordinate> {
        let mut antinodes: HashSet<Coordinate> = HashSet::new();
        let mut nodes: HashMap<char, HashSet<Coordinate>> = HashMap::new();

        self.grid.enumercells().for_each(|(coords, cell)| {
            // There should be only one value in the set
            let cell_value = cell.iter().next().unwrap();

            if *cell_value == '.' {
                return;
            }

            nodes
                .entry(*cell_value)
                .or_insert(HashSet::new())
                .insert(coords);
        });

        nodes.iter().for_each(|(_, node)| {
            let node_slice = node.iter().collect::<Vec<&Coordinate>>();

            for i in 0..node_slice.len() {
                for j in i + 1..node_slice.len() {
                    let node1 = node_slice[i];
                    let node2 = node_slice[j];

                    if nodes_in_direction > 1 {
                        antinodes.insert(node1.clone());
                        antinodes.insert(node2.clone());
                    }

                    let x_diff = node1.0 as isize - node2.0 as isize;
                    let y_diff = node1.1 as isize - node2.1 as isize;

                    // Top antinodes
                    for top_scaler in 1..=nodes_in_direction {
                        let top_antinode_x = node1.0 as isize + (x_diff * top_scaler as isize);
                        let top_antinode_y = node1.1 as isize + (y_diff * top_scaler as isize);

                        if top_antinode_x >= 0 && top_antinode_y >= 0 {
                            let antinode =
                                Coordinate(top_antinode_x as usize, top_antinode_y as usize);

                            if self.grid.is_in_bounds(&antinode) {
                                antinodes.insert(antinode);
                            }
                        }
                    }

                    // Bottom antinodes
                    for bottom_scaler in 1..=nodes_in_direction {
                        let bottom_antinode_x =
                            node2.0 as isize - (x_diff * bottom_scaler as isize);
                        let bottom_antinode_y =
                            node2.1 as isize - (y_diff * bottom_scaler as isize);

                        if bottom_antinode_x >= 0 && bottom_antinode_y >= 0 {
                            let antinode =
                                Coordinate(bottom_antinode_x as usize, bottom_antinode_y as usize);

                            if self.grid.is_in_bounds(&antinode) {
                                antinodes.insert(antinode);
                            }
                        }
                    }
                }
            }
        });

        antinodes
    }

    fn push_to_cell(&mut self, coordinate: Coordinate, value: char) -> () {
        let mut cell = self.grid.get(&coordinate).unwrap().clone();
        cell.insert(value);

        self.grid.set(&coordinate, cell);
    }
}
