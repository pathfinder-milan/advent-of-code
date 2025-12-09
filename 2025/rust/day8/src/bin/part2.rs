use day8::{multiply_last_two_junction_box_x_coordinates, parser};
use std::io;

fn main() -> io::Result<()> {
    let boxes = parser::parse_input("input.txt")?;
    let answer = multiply_last_two_junction_box_x_coordinates(&boxes);
    println!("{}", answer);
    Ok(())
}
