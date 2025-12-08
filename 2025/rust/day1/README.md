### Advent of Code Day 1: Secret Entrance
This folder contains the Rust solution for Day 1 of Advent of Code, structured into a library and separate binaries for Part 1 and Part 2.

---
### 📂 Project Structure
```
day1/
├── Cargo.toml
├── README.md
├── input.txt              # Not included
└── src/
    ├── lib.rs             # Contains : Direction, Command, simulate_commands(), get_the_password(), get_the_password_0x434_c49434_b()
    ├── parser.rs          # Contains : parse_input()
    └── bin/
        ├── part1.rs       # Part 1
        └── part2.rs       # Part 2
```
---
### Running the Solution

You must have your input data saved as ```input.txt``` in the root directory (day1/)

Answer 1

To get answer for part 1, call the part1 binary:

```bash
cargo run --bin part1
```


Answer 2

To get answer for part 2, call the part2 binary:
```bash
cargo run --bin part2
```
