use day11::{count_paths_from_you_to_out, parser};
use std::io;

fn main() -> io::Result<()> {
    let network = parser::parse_input("input.txt")?;
    let answer = count_paths_from_you_to_out(&network);
    println!("{}", answer);
    Ok(())
}
