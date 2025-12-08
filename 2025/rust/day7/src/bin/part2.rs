use day7::{how_many_timelines, parser};
use std::io;

fn main() -> io::Result<()> {
    let (grid, positions) = parser::parse_input("input.txt")?;

    let answer = how_many_timelines(&grid, &positions);
    println!("{}", answer);

    Ok(())
}
