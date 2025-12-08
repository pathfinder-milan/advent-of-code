use day5::{how_many_fresh, parser};
use std::io;

fn main() -> io::Result<()> {
    let data = parser::parse_input("input.txt")?;

    let answer = how_many_fresh(&data);
    println!("{}", answer);

    Ok(())
}
