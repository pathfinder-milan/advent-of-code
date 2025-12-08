use day3::{output_joltage_from_2, parser};
use std::io;

fn main() -> io::Result<()> {
    let data = parser::parse_input("input.txt")?;

    let answer = output_joltage_from_2(&data);

    println!("{}", answer);

    Ok(())
}
