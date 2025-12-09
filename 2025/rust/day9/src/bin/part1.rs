use day9::{largest_red_red_rect, parser};
use std::io;

fn main() -> io::Result<()> {
    let tiles = parser::parse_input("input.txt")?;
    let answer = largest_red_red_rect(&tiles);
    println!("{}", answer);
    Ok(())
}
