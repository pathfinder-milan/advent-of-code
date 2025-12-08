use day6::{parser, vertical_math_parser};
use std::io;

fn main() -> io::Result<()> {
    let data = parser::parse_input("input.txt")?;

    let answer = vertical_math_parser(&data);
    println!("{}", answer);

    Ok(())
}
