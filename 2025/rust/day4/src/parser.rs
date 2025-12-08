use crate::Grid;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<Grid> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.chars().collect())
            }
        })
        .collect();

    if grid.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Input grid is empty",
        ));
    }

    let rows = grid.len();
    let cols = grid[0].len();

    Ok(Grid { grid, rows, cols })
}
