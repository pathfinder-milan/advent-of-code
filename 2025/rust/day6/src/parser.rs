use crate::PuzzleData;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<PuzzleData> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut columns: Vec<Vec<i32>> = Vec::new();
    let mut results: Vec<i64> = Vec::new();
    let mut positions: Vec<Vec<usize>> = Vec::new();
    let mut symbols: Vec<String> = Vec::new();

    let mut spaces: Vec<Vec<usize>> = Vec::new();

    for line_result in reader.lines() {
        let starting_line = line_result?.to_string();

        let line = starting_line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        let line_with_numbers = parts.iter().all(|p| p.parse::<i32>().is_ok());

        if line_with_numbers {
            if columns.is_empty() {
                let num_parts = parts.len();
                columns.resize_with(num_parts, Vec::new);
                spaces.resize_with(num_parts.saturating_sub(1), Vec::new);
            }

            for (i, part) in parts.iter().enumerate() {
                if let Ok(number) = part.parse::<i32>() {
                    columns[i].push(number);
                }
            }

            let mut row_positions: Vec<usize> = Vec::new();
            let mut current_pos = 0;
            let raw_line = starting_line.as_str();

            for (i, current_number_str) in parts.iter().enumerate() {
                let start_of_current_number = match raw_line[current_pos..].find(current_number_str)
                {
                    Some(index) => index + current_pos,
                    None => continue,
                };

                let end_of_current_number = start_of_current_number + current_number_str.len();

                row_positions.push(start_of_current_number);
                current_pos = end_of_current_number;

                if i < parts.len().saturating_sub(1) {
                    let next_number_str = parts[i + 1];
                    let start_of_next_number = match raw_line[current_pos..].find(next_number_str) {
                        Some(next_index) => next_index + current_pos,
                        None => continue,
                    };

                    let space_count = start_of_next_number - end_of_current_number;
                    spaces[i].push(space_count);
                    current_pos = start_of_next_number;
                }
            }

            positions.push(row_positions);
        } else {
            for (i, symbol) in parts.into_iter().enumerate() {
                if i < columns.len() {
                    let column = &columns[i];
                    let result = crate::calculate_result(column, symbol);
                    results.push(result);
                    symbols.push(symbol.to_string());
                }
            }
        }
    }

    Ok(PuzzleData {
        columns,
        symbols,
        results,
        positions,
    })
}
