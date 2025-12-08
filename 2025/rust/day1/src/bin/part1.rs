use day1::{get_the_password, parser};
use std::io;

fn main() -> io::Result<()> {
    const START_POS: i32 = 50;

    let commands = parser::parse_input("input.txt")?;

    let answer = get_the_password(&commands, START_POS);

    println!("{}", answer);

    Ok(())
}
