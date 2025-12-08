use day4::{how_many_removed, parser};
use std::io;

fn main() -> io::Result<()> {
    let data = parser::parse_input("input.txt")?;

    let answer = how_many_removed(&data);

    println!("{}", answer);

    Ok(())
}
