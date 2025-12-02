use std::fs::File;
use std::io::{self, BufReader, BufRead};

type Range = (i64, i64);


fn is_two_block_repeat(n: i64) -> bool {

    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half_len = len / 2;
    let first_half = &s[..half_len];
    let second_half = &s[half_len..];

    first_half == second_half
}

fn is_more_block_repeat(n: i64) -> bool {
    let s = n.to_string();
    let total_length = s.len();

    let last_digit_char = s.chars().last().unwrap();
    
    for block_length in 1..=total_length / 2 {
        
        let potential_block_end_char = s.chars().nth(block_length - 1).unwrap();
        if potential_block_end_char != last_digit_char {
            continue;
        }

        if total_length % block_length != 0 {
            continue;
        }

        let block_s = &s[..block_length];
        let num_repeats = total_length / block_length;
        
        let mut is_repeating = true;
        
        for repeat_index in 1..num_repeats {
            let start_pos = repeat_index * block_length;
            let end_pos = start_pos + block_length;
            
            let current_block = &s[start_pos..end_pos];
            
            if current_block != block_s {
                is_repeating = false;
                break; 
            }
        }
        
        if is_repeating { return true; }
    }
    
    return false;
}

fn main() -> Result<(), io::Error> {

    let file_path = "input.txt"; 
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    let line_content: String = line.trim_end().to_string();

    let ranges_vector: Vec<Range> = line_content
        .split(',')
        .filter_map(|segment| {
            let parts: Vec<&str> = segment.split('-').collect();
            if parts.len() != 2 { return None; }

            match (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                (Ok(start), Ok(end)) => Some((start, end)),
                _ => None,
            }
        })
        .collect();

    let mut part1_ids: Vec<i64> = Vec::new();
    let mut part2_ids: Vec<i64> = Vec::new();

    for (start, end) in ranges_vector {
        for number in start..=end {
            if is_two_block_repeat(number) {
                part1_ids.push(number);
            }
            
            if is_more_block_repeat(number) {
                part2_ids.push(number);
            }
        }
    }

    let answer1: i64 = part1_ids.iter().sum();
    println!("Part 1 : {}", answer1);

    let answer2: i64 = part2_ids.iter().sum();
    println!("Part 2 : {}", answer2);

    Ok(())
}