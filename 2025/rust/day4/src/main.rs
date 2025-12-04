use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn remove_all(
    grid: &[Vec<char>],
    neighbors: &mut [Vec<usize>],
    removed: &mut [Vec<bool>],
    rows: usize,
    cols: usize,
    directions: &[(i32, i32)]
) -> usize {
    let mut removed_total = 0;
    
    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '@' 
                    && !removed[row][col] 
                    && neighbors[row][col] < 4 {
                    to_remove.push((row, col));
                }
            }
        }
        
        if to_remove.is_empty() { break; }
        
        for (row, col) in to_remove.iter() {
            removed[*row][*col] = true;
            neighbors[*row][*col] = 0;
            removed_total += 1;
            
            for (dr, dc) in directions.iter() {
                let new_row = *row as i32 + dr;
                let new_col = *col as i32 + dc;
                
                if new_row >= 0 && new_row < rows as i32 
                    && new_col >= 0 && new_col < cols as i32 {
                    let nr = new_row as usize;
                    let nc = new_col as usize;
                    
                    if grid[nr][nc] == '@' && !removed[nr][nc] {
                        neighbors[nr][nc] = neighbors[nr][nc].saturating_sub(1);
                    }
                }
            }
        }
    }
    
    removed_total
}


fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    
    let rows = grid.len();
    let cols = grid[0].len();

    let mut neighbors: Vec<Vec<usize>> = vec![vec![0; cols]; rows];

    let forward_directions = [(0, 1), (1, -1), (1, 0), (1, 1)];
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '@' {
                for (dr, dc) in forward_directions.iter() {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;
                    
                    if new_row >= 0 && new_row < rows as i32 
                        && new_col >= 0 && new_col < cols as i32 {
                        let nr = new_row as usize;
                        let nc = new_col as usize;
                        
                        if grid[nr][nc] == '@' {
                            neighbors[row][col] += 1;
                            neighbors[nr][nc] += 1;
                        }
                    }
                }
            }
        }
    }

    // Part 1
    let mut removed_once = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '@' && neighbors[row][col] < 4 {
                removed_once += 1;
            }
        }
    }

    println!("Part 1: {}", removed_once);

    // Part 2
    let mut removed: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    
    let removed_total = remove_all(
        &grid, 
        &mut neighbors, 
        &mut removed, 
        rows, 
        cols, 
        &directions
    );

    println!("Part 2: {}", removed_total);

    Ok(())
}