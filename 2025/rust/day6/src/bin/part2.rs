use day6::{parser, vertical_right_left_parser};
use std::io;

fn main() -> io::Result<()> {
    let data = parser::parse_input("input.txt")?;

    let answer = vertical_right_left_parser(&data);
    println!("{}", answer);

    Ok(())
}
