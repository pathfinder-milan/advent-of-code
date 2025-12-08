use crate::DigitLines;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<DigitLines> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines: Vec<Vec<u32>> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    Ok(DigitLines { lines })
}
