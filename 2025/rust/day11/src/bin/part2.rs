use day11::{count_paths_from_svr_to_out_through_dac_and_fft, parser};
use std::io;

fn main() -> io::Result<()> {
    let network = parser::parse_input("input.txt")?;
    let answer = count_paths_from_svr_to_out_through_dac_and_fft(&network);
    println!("{}", answer);
    Ok(())
}
