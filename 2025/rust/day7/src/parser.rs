use crate::{Grid, Position};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<(Grid, Vec<Position>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut grid: Grid = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?.trim().to_string();
        if line.is_empty() {
            continue;
        }

        grid.push(line.chars().collect());
    }

    let positions: Vec<Position> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, &ch)| if ch == 'S' { Some((r, c)) } else { None })
        })
        .collect();

    Ok((grid, positions))
}
