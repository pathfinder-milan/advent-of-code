use day1::{get_the_password_0x434_c49434_b, parser};
use std::io;

fn main() -> io::Result<()> {
    const START_POS: i32 = 50;

    let commands = parser::parse_input("input.txt")?;

    let answer = get_the_password_0x434_c49434_b(&commands, START_POS);

    println!("{}", answer);

    Ok(())
}
