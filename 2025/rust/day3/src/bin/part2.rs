use day3::{output_joltage_from_12, parser};
use std::io;

fn main() -> io::Result<()> {
    let data = parser::parse_input("input.txt")?;

    let answer = output_joltage_from_12(&data);

    println!("{}", answer);

    Ok(())
}
