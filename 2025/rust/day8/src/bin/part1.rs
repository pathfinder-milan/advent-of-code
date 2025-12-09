use day8::{multiply_three_largest_circuit_sizes, parser};
use std::io;

fn main() -> io::Result<()> {
    let boxes = parser::parse_input("input.txt")?;
    let answer = multiply_three_largest_circuit_sizes(&boxes, 1000);
    println!("{}", answer);
    Ok(())
}
