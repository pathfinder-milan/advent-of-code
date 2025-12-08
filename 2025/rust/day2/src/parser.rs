use crate::BlockData;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub type Range = (i64, i64);

pub fn parse_input(filename: &str) -> io::Result<BlockData> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    let line_content: String = line.trim_end().to_string();

    let ranges_vector: Vec<Range> = line_content
        .split(',')
        .filter_map(|segment| {
            let parts: Vec<&str> = segment.split('-').collect();
            if parts.len() != 2 {
                return None;
            }

            match (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                (Ok(start), Ok(end)) => Some((start, end)),
                _ => None,
            }
        })
        .collect();

    if ranges_vector.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No valid ranges found in input.txt",
        ));
    }

    Ok(BlockData {
        ranges: ranges_vector,
    })
}
