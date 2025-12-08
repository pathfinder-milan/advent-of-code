use crate::{Command, Direction};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<Vec<Command>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut commands = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            continue;
        }

        let direction = match line.chars().next() {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => {
                eprintln!("Error: Invalid direction marker in '{}'.", line);
                continue;
            }
        };

        let value: i32 = match line.get(1..).and_then(|s| s.parse().ok()) {
            Some(num) => num,
            None => {
                eprintln!("Error: Failed to parse distance value in '{}'.", line);
                continue;
            }
        };

        commands.push(Command { direction, value });
    }

    Ok(commands)
}
