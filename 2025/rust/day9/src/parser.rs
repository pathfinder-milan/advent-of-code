use crate::Tile;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<Vec<Tile>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut tiles = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let coords: Vec<&str> = trimmed.split(',').collect();
        if coords.len() == 2 {
            if let (Ok(x), Ok(y)) = (coords[0].parse::<i32>(), coords[1].parse::<i32>()) {
                tiles.push(Tile::new(x, y));
            }
        }
    }

    Ok(tiles)
}
