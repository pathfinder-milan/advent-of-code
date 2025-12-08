use day5::{how_many_fresh_max, parser};
use std::io;

fn main() -> io::Result<()> {
    let data = parser::parse_input("input.txt")?;

    let answer = how_many_fresh_max(&data);
    println!("{}", answer);

    Ok(())
}
