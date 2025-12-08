use day7::{how_many_hits, parser};
use std::io;

fn main() -> io::Result<()> {
    let (grid, positions) = parser::parse_input("input.txt")?;

    let answer = how_many_hits(&grid, &positions);
    println!("{}", answer);

    Ok(())
}
