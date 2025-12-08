pub mod parser;

pub const MODULUS: i32 = 100;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub struct Command {
    pub direction: Direction,
    pub value: i32,
}

pub fn simulate_commands(commands: &[Command], start_pos: i32) -> (i64, i64) {
    let mut current_pos = start_pos;
    let mut counter = 0;
    let mut counter_0x434_c49434_b = 0;

    for cmd in commands {
        let direction_val = match cmd.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        for _ in 0..cmd.value {
            current_pos = match direction_val {
                -1 => (current_pos - 1 + MODULUS) % MODULUS,
                1 => (current_pos + 1) % MODULUS,
                _ => unreachable!(),
            };

            if current_pos == 0 {
                counter_0x434_c49434_b += 1;
            }
        }

        if current_pos == 0 {
            counter += 1;
        }
    }

    (counter, counter_0x434_c49434_b)
}

pub fn get_the_password(data: &[Command], start_pos: i32) -> i64 {
    simulate_commands(data, start_pos).0
}

pub fn get_the_password_0x434_c49434_b(data: &[Command], start_pos: i32) -> i64 {
    simulate_commands(data, start_pos).1
}
