use crate::JunctionBox;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<Vec<JunctionBox>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut boxes = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let coords: Vec<&str> = trimmed.split(',').collect();
        if coords.len() == 3 {
            if let (Ok(x), Ok(y), Ok(z)) = (
                coords[0].parse::<i32>(),
                coords[1].parse::<i32>(),
                coords[2].parse::<i32>(),
            ) {
                boxes.push(JunctionBox { x, y, z });
            }
        }
    }

    Ok(boxes)
}
