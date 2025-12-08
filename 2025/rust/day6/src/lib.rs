pub mod parser;

use std::collections::HashMap;

pub struct PuzzleData {
    pub columns: Vec<Vec<i32>>,
    pub symbols: Vec<String>,
    pub results: Vec<i64>,
    pub positions: Vec<Vec<usize>>,
}

pub fn calculate_result(column: &[i32], symbol: &str) -> i64 {
    match symbol {
        "+" => column.iter().map(|&n| n as i64).sum(),
        "*" => column.iter().map(|&n| n as i64).product(),
        _ => 0,
    }
}

pub fn vertical_math_parser(data: &PuzzleData) -> i64 {
    data.results.iter().sum()
}

pub fn vertical_right_left_parser(data: &PuzzleData) -> i64 {
    if data.positions.is_empty() || data.columns.is_empty() {
        return 0;
    }

    let num_columns = data.columns.len();
    let mut column_by_column_positions: Vec<Vec<usize>> = Vec::new();
    column_by_column_positions.resize_with(num_columns, Vec::new);

    for line_positions in &data.positions {
        for col_idx in 0..num_columns {
            if col_idx < line_positions.len() {
                column_by_column_positions[col_idx].push(line_positions[col_idx]);
            }
        }
    }

    let mut digit_maps: Vec<HashMap<usize, String>> = Vec::new();
    digit_maps.resize_with(num_columns, HashMap::new);

    for (col_idx, column) in data.columns.iter().enumerate() {
        let digit_map = &mut digit_maps[col_idx];

        for (row_idx, &number) in column.iter().enumerate() {
            let start_pos = column_by_column_positions[col_idx][row_idx];
            let number_str = number.to_string();

            for (offset, ch) in number_str.chars().enumerate() {
                let pos = start_pos + offset;
                digit_map.entry(pos).or_insert_with(String::new).push(ch);
            }
        }
    }

    let mut vertical_numbers: Vec<Vec<i32>> = Vec::new();

    for digit_map in digit_maps.iter() {
        let mut numbers: Vec<i32> = Vec::new();

        let mut sorted_positions: Vec<_> = digit_map.keys().cloned().collect();
        sorted_positions.sort_unstable();

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
        if col_idx < data.symbols.len() {
            let symbol = &data.symbols[col_idx];
            let result = calculate_result(numbers, symbol);
            vertical_results.push(result);
        }
    }

    vertical_results.iter().sum()
}
