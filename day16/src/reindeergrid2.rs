use std::cmp::Ordering;

use common::coordinate::Coordinate;
use common::grid::{Grid, GridDirection};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    pub coord: Coordinate,
    dir: GridDirection,
}

type StateKey = (Coordinate, GridDirection);

// We want the lowest cost at the top, so we invert the ordering on cost.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // If we did self.cost.cmp(&other.cost), we’d get a max-heap
        // So we flip it to get a min-heap
        other.cost.cmp(&self.cost).then_with(|| {
            self.coord
                .partial_cmp(&other.coord)
                .unwrap_or(Ordering::Equal)
        })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

pub struct ReindeerGrid2 {
    grid: Grid<GridCell>,
}

impl ReindeerGrid2 {
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
        self.dijkstra().0
    }

    pub fn get_num_coords_on_lowest(&self) -> u64 {
        self.dijkstra().1.len() as u64
    }

    pub fn dijkstra(&self) -> (u64, HashSet<Coordinate>) {
        // 1. Find start and end
        let start_coord = self.get_start();
        let end_coord = self.get_end();

        // Store parents/predecessors
        let mut parents: HashMap<StateKey, Vec<StateKey>> = HashMap::new();

        // 2. Initialize the dist map
        let mut dist: HashMap<(Coordinate, GridDirection), u64> = HashMap::new();
        // For all possible states, dist is initially "infinity"
        // but we fill lazily only as we discover states

        // 3. Priority queue
        let mut heap = BinaryHeap::new();

        // 4. Insert the start into the heap (assuming we want to start facing Right)
        dist.insert((start_coord, GridDirection::Right), 0);
        heap.push(State {
            cost: 0,
            coord: start_coord,
            dir: GridDirection::Right,
        });

        // 5. Dijkstra Loop
        while let Some(State { cost, coord, dir }) = heap.pop() {
            // If this is out of date (we found a better path), skip
            if dist.get(&(coord, dir)).copied().unwrap_or(u64::MAX) < cost {
                continue;
            }

            // If we've reached the end coordinate, return cost
            if coord == end_coord {
                let mut all_paths = Vec::new();
                let mut path_so_far = Vec::new();

                // Suppose the best direction at end was end_dir (or you might store a set of end directions)
                Self::backtrack_all_paths(
                    &parents,
                    (end_coord, GridDirection::Up),
                    (start_coord, GridDirection::Right),
                    &mut path_so_far,
                    &mut all_paths,
                );

                Self::backtrack_all_paths(
                    &parents,
                    (end_coord, GridDirection::Right),
                    (start_coord, GridDirection::Right),
                    &mut path_so_far,
                    &mut all_paths,
                );

                return (
                    cost,
                    all_paths
                        .iter()
                        .flat_map(|path| path.iter().map(|state| state.0))
                        .collect(),
                );
            }

            // Explore neighbors
            for adj in self.grid.get_adjacent(&coord) {
                // Possibly skip walls
                if self.grid.get(&adj) == Some(&GridCell::Wall) {
                    continue;
                }

                // Decide if it’s going straight or turning
                if Self::is_going_same_direction(&coord, &adj, dir) {
                    // Going straight
                    let next_cost = cost.saturating_add(1);
                    self.relax(
                        &mut dist,
                        &mut heap,
                        &mut parents,
                        adj,
                        dir,
                        coord,
                        dir,
                        next_cost,
                    );
                } else {
                    let (num_turns, turn_dir) = Self::get_turns_to_face_dir(&coord, &adj, dir);
                    let new_dir = Self::apply_turns(dir, num_turns, turn_dir);
                    let next_cost = cost.saturating_add(num_turns as u64 * 1000 + 1);
                    self.relax(
                        &mut dist,
                        &mut heap,
                        &mut parents,
                        adj,
                        new_dir,
                        coord,
                        dir,
                        next_cost,
                    );
                }
            }
        }

        // If we exhaust the heap without reaching end, no path broseph
        (u64::MAX, HashSet::new())
    }

    fn relax(
        &self,
        dist: &mut HashMap<(Coordinate, GridDirection), u64>,
        heap: &mut BinaryHeap<State>,
        parents: &mut HashMap<StateKey, Vec<StateKey>>,
        next_coord: Coordinate,
        next_dir: GridDirection,
        coord: Coordinate,
        dir: GridDirection,
        cost: u64,
    ) {
        // 1) Check the old cost of the "new" state (next_coord, next_dir), not the old one
        let old_cost = dist
            .get(&(next_coord, next_dir))
            .copied()
            .unwrap_or(u64::MAX);

        // 2) If the new cost is strictly better, update dist and push the new state
        if cost < old_cost {
            dist.insert((next_coord, next_dir), cost);

            // Record that (coord, dir) is a parent of (next_coord, next_dir)
            parents.insert((next_coord, next_dir), vec![(coord, dir)]);

            // Push the new state, not the old one
            heap.push(State {
                cost,
                coord: next_coord,
                dir: next_dir,
            });
        } else if cost == old_cost {
            // Found an equally good route
            if let Some(vec_of_parents) = parents.get_mut(&(next_coord, next_dir)) {
                vec_of_parents.push((coord, dir));
            }
            // No need to push into the heap again, because the cost is the same.
        }
    }

    fn backtrack_all_paths(
        parents: &HashMap<StateKey, Vec<StateKey>>,
        current: StateKey,
        start: StateKey,
        path_so_far: &mut Vec<StateKey>,
        all_paths: &mut Vec<Vec<StateKey>>,
    ) {
        // If we've reached the start
        if current == start {
            path_so_far.push(current);
            // clone the path and reverse it to get start->end
            let mut final_path = path_so_far.clone();
            final_path.reverse();
            all_paths.push(final_path);
            path_so_far.pop();
            return;
        }

        // Not at start; see if we have parents
        if let Some(parents_of_current) = parents.get(&current) {
            path_so_far.push(current);
            // Explore each parent
            for &p in parents_of_current {
                Self::backtrack_all_paths(parents, p, start, path_so_far, all_paths);
            }
            path_so_far.pop();
        } else {
            // No parents - no path
        }
    }

    fn apply_turns(dir: GridDirection, num_turns: usize, turn_dir: TurnDirection) -> GridDirection {
        let mut new_dir = dir;
        for _ in 0..num_turns {
            new_dir = match turn_dir {
                TurnDirection::Clockwise => new_dir.turn_clockwise(),
                TurnDirection::CounterClockwise => new_dir.turn_counter_clockwise(),
            };
        }
        new_dir
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
        self.grid.find(&GridCell::Start)[0]
    }

    fn get_end(&self) -> Coordinate {
        self.grid.find(&GridCell::End)[0]
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
