pub mod parser;

pub struct Grid {
    pub grid: Vec<Vec<char>>,
    pub rows: usize,
    pub cols: usize,
}

const FORWARD_DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, -1), (1, 0), (1, 1)];

const ALL_DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn calculate_initial_neighbors(data: &Grid) -> Vec<Vec<usize>> {
    let mut neighbors: Vec<Vec<usize>> = vec![vec![0; data.cols]; data.rows];

    for r in 0..data.rows {
        for c in 0..data.cols {
            if data.grid[r][c] == '@' {
                for (dr, dc) in FORWARD_DIRECTIONS.iter() {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;

                    if nr >= 0 && nr < data.rows as i32 && nc >= 0 && nc < data.cols as i32 {
                        let nr = nr as usize;
                        let nc = nc as usize;

                        if data.grid[nr][nc] == '@' {
                            neighbors[r][c] += 1;
                            neighbors[nr][nc] += 1;
                        }
                    }
                }
            }
        }
    }
    neighbors
}

pub fn remove_all(data: &Grid, initial_neighbors: &mut Vec<Vec<usize>>) -> usize {
    let mut neighbors = initial_neighbors.clone();
    let mut removed: Vec<Vec<bool>> = vec![vec![false; data.cols]; data.rows];
    let mut removed_total = 0;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for r in 0..data.rows {
            for c in 0..data.cols {
                if data.grid[r][c] == '@' && !removed[r][c] && neighbors[r][c] < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove.iter() {
            removed[*r][*c] = true;
            neighbors[*r][*c] = 0;
            removed_total += 1;

            for (dr, dc) in ALL_DIRECTIONS.iter() {
                let nr = *r as i32 + dr;
                let nc = *c as i32 + dc;

                if nr >= 0 && nr < data.rows as i32 && nc >= 0 && nc < data.cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if data.grid[nr][nc] == '@' && !removed[nr][nc] {
                        neighbors[nr][nc] = neighbors[nr][nc].saturating_sub(1);
                    }
                }
            }
        }
    }

    removed_total
}

pub fn how_many_removed_once(data: &Grid, initial_neighbors: &[Vec<usize>]) -> usize {
    let mut removed_once = 0;
    for r in 0..data.rows {
        for c in 0..data.cols {
            if data.grid[r][c] == '@' && initial_neighbors[r][c] < 4 {
                removed_once += 1;
            }
        }
    }
    removed_once
}

pub fn how_many_removed(data: &Grid) -> usize {
    let mut initial_neighbors = calculate_initial_neighbors(data);
    remove_all(data, &mut initial_neighbors)
}
