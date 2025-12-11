### Advent of Code Day 10: Factory
This folder contains the Rust solution for Day 10 of Advent of Code, structured into a library and separate binaries for Part 1 and Part 2.

---
### 📂 Project Structure
```
day10/
├── Cargo.toml
├── README.md
├── input.txt              # Not included
└── src/
    ├── lib.rs             # Contains : Machine, min_button_presses_to_configure_lights(), min_button_presses_to_configure_lights_all()
    ├── parser.rs          # Contains : parse_input()
    └── bin/
        └──part1.rs       # Part 1
```
---
### Running the Solution

You must have your input data saved as ```input.txt``` in the root directory (day10/)

Answer 1

To get answer for part 1, call the part1 binary:

```bash
cargo run --bin part1
```
