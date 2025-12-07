use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn calculate_result(column: &[i32], symbol: &str) -> i64 {
    match symbol {
        "+" => column.iter().map(|&n| n as i64).sum(),
        "*" => column.iter().map(|&n| n as i64).product(),
        _ => 0,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut columns: Vec<Vec<i32>> = Vec::new();
    let mut results: Vec<i64> = Vec::new();
    let mut spaces: Vec<Vec<usize>> = Vec::new();
    let mut positions: Vec<Vec<usize>> = Vec::new();
    let mut symbols: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let starting_line = line_result?.to_string();
        let line = starting_line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let line_with_numbers = parts.iter().all(|p| p.parse::<i32>().is_ok());

        if line_with_numbers {
            if columns.is_empty() {
                let num_parts = parts.len();
                for _ in 0..num_parts {
                    columns.push(Vec::new());
                }

                for _ in 0..num_parts.saturating_sub(1) {
                    spaces.push(Vec::new());
                }
            }

            for (i, part) in parts.iter().enumerate() {
                let number = part.parse::<i32>().unwrap();
                columns[i].push(number);
            }

            let mut current_pos = 0;
            let current_line = starting_line.as_str();

            for i in 0..parts.len().saturating_sub(1) {
                let current_number_str = parts[i];
                let next_number_str = parts[i + 1];

                let start_of_current_number =
                    match current_line[current_pos..].find(current_number_str) {
                        Some(index) => index + current_pos,
                        None => continue,
                    };

                let end_of_current_number = start_of_current_number + current_number_str.len();
                current_pos = end_of_current_number;

                let start_of_next_number = match current_line[current_pos..].find(next_number_str) {
                    Some(next_index) => next_index + current_pos,
                    None => continue,
                };

                let space_count = start_of_next_number - end_of_current_number;
                spaces[i].push(space_count);
                current_pos = start_of_next_number;
            }

            let mut pos: Vec<usize> = Vec::new();
            pos.push(0);

            for i in 0..parts.len() - 1 {
                let prev_number_len = parts[i].len();
                let space_between = spaces[i].last().cloned().unwrap_or(0);

                let next_pos = pos[i] + prev_number_len + space_between;
                pos.push(next_pos);
            }

            positions.push(pos);
        } else {
            for (i, symbol) in parts.into_iter().enumerate() {
                let column = &columns[i];
                let result = calculate_result(column, symbol);
                results.push(result);
                symbols.push(symbol.to_string());
            }
        }
    }

    // Part 1

    let answer1: i64 = results.iter().sum();
    println!("Part 1: {}", answer1);

    // Part 2

    let mut column_by_column_positions: Vec<Vec<usize>> = Vec::new();

    if !positions.is_empty() {
        let num_columns = positions[0].len();

        for col_idx in 0..num_columns {
            let mut column_positions = Vec::new();
            for line_positions in &positions {
                if col_idx < line_positions.len() {
                    column_positions.push(line_positions[col_idx]);
                }
            }
            column_by_column_positions.push(column_positions);
        }
    }

    let mut digit_maps: Vec<HashMap<usize, String>> = Vec::new();

    for (col_idx, column) in columns.iter().enumerate() {
        let mut digit_map: HashMap<usize, String> = HashMap::new();

        for (row_idx, &number) in column.iter().enumerate() {
            let start_pos = column_by_column_positions[col_idx][row_idx];
            let number_str = number.to_string();

            for (offset, ch) in number_str.chars().enumerate() {
                let pos = start_pos + offset;
                digit_map.entry(pos).or_insert_with(String::new).push(ch);
            }
        }

        digit_maps.push(digit_map);
    }

    let mut vertical_numbers: Vec<Vec<i32>> = Vec::new();

    for digit_map in digit_maps.iter() {
        let mut numbers: Vec<i32> = Vec::new();

        let mut sorted_positions: Vec<_> = digit_map.keys().cloned().collect();
        sorted_positions.sort();

        for position in sorted_positions {
            if let Some(digit_str) = digit_map.get(&position) {
                if let Ok(num) = digit_str.parse::<i32>() {
                    numbers.push(num);
                }
            }
        }

        vertical_numbers.push(numbers);
    }

    let mut vertical_results: Vec<i64> = Vec::new();

    for (col_idx, numbers) in vertical_numbers.iter().enumerate() {
        if col_idx < symbols.len() {
            let symbol = &symbols[col_idx];
            let result = calculate_result(numbers, symbol);
            vertical_results.push(result);
        }
    }

    let answer2: i64 = vertical_results.iter().sum();
    println!("Part 2: {}", answer2);

    Ok(())
}
