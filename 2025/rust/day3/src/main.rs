use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn find_n_biggest(digits: &[u32], n: usize) -> Vec<u32> {
    match n {
        0 => return Vec::new(),
        n if n >= digits.len() => return digits.to_vec(),
        _ => {}
    }
    
    let batteries = digits.len();
    let mut sequence = Vec::with_capacity(n);
    let mut iterator = 0;
        
    while sequence.len() < n {
        let remaining = n - sequence.len();
        
        if batteries - iterator == remaining {
            sequence.extend_from_slice(&digits[iterator..iterator + remaining]);
            break;
        }

        let max_index = batteries - remaining;
        let max_digit = digits[iterator..=max_index].iter().max().unwrap();
        
        if let Some(offset) = digits[iterator..=max_index]
            .iter()
            .position(|&d| d == *max_digit) 
        {
            sequence.push(*max_digit);
            iterator += offset + 1;
        }
    }

    sequence
}

fn main() -> io::Result<()> {
    let (n1, n2) = (2, 12);
    let (mut answer1, mut answer2) = (0u64, 0u64);

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines().filter_map(Result::ok).filter(|l| !l.is_empty()) {
        let digits: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        for (n, answer) in [(n1, &mut answer1), (n2, &mut answer2)] {
            if let Ok(number) = find_n_biggest(&digits, n)
                .iter()
                .map(|d| d.to_string())
                .collect::<String>()
                .parse::<u64>() 
            {
                *answer += number;
            }
        }
    }

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    Ok(())
}