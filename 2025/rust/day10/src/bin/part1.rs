use day10::{min_button_presses_to_configure_lights_all, parser};
use std::io;

fn main() -> io::Result<()> {
    let machines = match parser::parse_input("input.txt") {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error reading or parsing input: {}", e);
            return Err(e);
        }
    };

    let answer = min_button_presses_to_configure_lights_all(&machines);

    println!("{}", answer);

    Ok(())
}
