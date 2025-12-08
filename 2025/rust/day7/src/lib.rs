pub mod parser;

use std::collections::{HashMap, HashSet, VecDeque};

pub type Position = (usize, usize);
pub type Grid = Vec<Vec<char>>;

pub fn how_many_hits(grid: &Grid, positions: &[Position]) -> usize {
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut queue: VecDeque<Position> = VecDeque::new();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut hits: HashSet<Position> = HashSet::new();
    let mut count: usize = 0;

    for &start_position in positions {
        if visited.insert(start_position) {
            queue.push_back(start_position);
        }
    }

    while let Some(current_position) = queue.pop_front() {
        let (_initial_r, c) = current_position;
        let mut r = current_position.0 + 1;

        while r < nrows {
            let current_pos: Position = (r, c);

            if grid[current_pos.0][current_pos.1] == '^' {
                if hits.insert(current_pos) {
                    count += 1;
                }

                if c > 0 {
                    let neighbor_left = (r, c - 1);
                    if visited.insert(neighbor_left) {
                        queue.push_back(neighbor_left);
                    }
                }

                if c + 1 < ncols {
                    let neighbor_right = (r, c + 1);
                    if visited.insert(neighbor_right) {
                        queue.push_back(neighbor_right);
                    }
                }
                break;
            }
            r += 1;
        }
    }
    count
}

pub fn how_many_timelines(grid: &Grid, positions: &[Position]) -> usize {
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut memory: HashMap<Position, usize> = HashMap::new();

    let mut count = 0;
    for &start_pos in positions {
        count += dfs(grid, start_pos.0, start_pos.1, nrows, ncols, &mut memory);
    }
    count
}

fn dfs(
    grid: &Grid,
    row: usize,
    col: usize,
    nrows: usize,
    ncols: usize,
    memory: &mut HashMap<Position, usize>,
) -> usize {
    let position = (row, col);

    if let Some(&cached) = memory.get(&position) {
        return cached;
    }

    if row == nrows - 1 {
        memory.insert(position, 1);
        return 1;
    }

    let mut current_row = row + 1;

    while current_row < nrows {
        if grid[current_row][col] == '^' {
            let mut timelines = 0;

            if col > 0 {
                timelines += dfs(grid, current_row, col - 1, nrows, ncols, memory);
            }
            if col + 1 < ncols {
                timelines += dfs(grid, current_row, col + 1, nrows, ncols, memory);
            }

            memory.insert(position, timelines);
            return timelines;
        }
        current_row += 1;
    }

    memory.insert(position, 1);
    1
}
