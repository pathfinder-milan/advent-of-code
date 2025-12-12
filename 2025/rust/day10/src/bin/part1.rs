use day10::{min_button_presses_to_configure_lights_all, parser};
use std::io;

fn main() -> io::Result<()> {
    let machines = parser::parse_input("input.txt")?;
    let answer = min_button_presses_to_configure_lights_all(&machines);
    println!("{}", answer);
    Ok(())
}
