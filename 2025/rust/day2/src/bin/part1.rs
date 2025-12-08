use day2::{how_many_invalid_ids_with_2_invalid_blocks, parser};
use std::io;

fn main() -> io::Result<()> {
    let data = parser::parse_input("input.txt")?;

    let answer = how_many_invalid_ids_with_2_invalid_blocks(&data);

    println!("{}", answer);

    Ok(())
}
