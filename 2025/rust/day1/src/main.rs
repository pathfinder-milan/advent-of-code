use std::fs::File;
use std::io::{self, BufReader, BufRead};

const MOD: i32 = 100;

fn main() -> Result<(), io::Error> {
    let mut iterator = 50;
    let mut counter1 = 0;
    let mut counter2 = 0;

    let file_path = "input.txt"; 
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        if line.is_empty() { continue; }

        let direction: i32 = match line.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => { eprintln!("Error: Invalid direction: '{}'.", line); continue; }
        };

        let value: i32 = match line.get(1..).and_then(|s| s.parse().ok()) {
            Some(num) => num,
            None => { eprintln!("Error: Failed to parse distance value: '{}'.", line); continue; }
        };

        for _ in 0..value {
            match direction {
                -1 => {
                    iterator = if iterator == 0 { MOD - 1 } else { iterator - 1 };
                },
                1 => {
                    iterator = (iterator + 1) % MOD;
                },
                _ => unreachable!(),
            }

            if iterator == 0 {
                counter2 += 1;
            }
        }

        if iterator == 0 {
            counter1 += 1;
        }
    }
    
    println!("Part 1 (Final Hits): {}", counter1);
    println!("Part 2 (Total Events): {}", counter2);

    Ok(())
}