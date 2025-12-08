use crate::{Ranges, fix_ranges};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<Ranges> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut start_range: Vec<u64> = Vec::new();
    let mut end_range: Vec<u64> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    let mut parsing_ranges = true;

    for line_result in reader.lines() {
        let line = line_result?.trim().to_string();

        if line.is_empty() {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            if let Some((start_str, end_str)) = line.split_once('-') {
                if let (Ok(s), Ok(e)) = (start_str.parse::<u64>(), end_str.parse::<u64>()) {
                    fix_ranges(&mut start_range, &mut end_range, s, e);
                }
            }
        } else {
            if let Ok(id) = line.parse::<u64>() {
                ids.push(id);
            }
        }
    }

    Ok(Ranges {
        start_range,
        end_range,
        ids,
    })
}
