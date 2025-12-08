use day4::{calculate_initial_neighbors, how_many_removed_once, parser};
use std::io;

fn main() -> io::Result<()> {
    let data = parser::parse_input("input.txt")?;
    let initial_neighbors = calculate_initial_neighbors(&data);

    let answer = how_many_removed_once(&data, &initial_neighbors);

    println!("{}", answer);

    Ok(())
}
