pub mod parser;

pub struct BlockData {
    pub ranges: Vec<(i64, i64)>,
}

fn check_2_exact_blocks(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half_len = len / 2;
    let first_half = &s[..half_len];
    let second_half = &s[half_len..];

    first_half == second_half
}

fn check_n_exact_blocks(n: i64) -> bool {
    let s = n.to_string();
    let total_length = s.len();

    if total_length < 2 {
        return false;
    }

    if check_2_exact_blocks(n) {
        return true;
    }

    for block_length in 1..=total_length / 2 {
        if total_length % block_length != 0 {
            continue;
        }

        let block_s = &s[..block_length];
        let num_repeats = total_length / block_length;

        let mut is_repeating = true;

        for repeat_index in 1..num_repeats {
            let start_pos = repeat_index * block_length;
            let end_pos = start_pos + block_length;

            let current_block = &s[start_pos..end_pos];

            if current_block != block_s {
                is_repeating = false;
                break;
            }
        }

        if is_repeating {
            return true;
        }
    }
    false
}

pub fn how_many_invalid_ids_with_2_invalid_blocks(data: &BlockData) -> i64 {
    let mut count: i64 = 0;

    for (start, end) in &data.ranges {
        for number in *start..=*end {
            if check_2_exact_blocks(number) {
                count += number;
            }
        }
    }
    count
}

pub fn how_many_invalid_ids_with_n_invalid_blocks(data: &BlockData) -> i64 {
    let mut count: i64 = 0;

    for (start, end) in &data.ranges {
        for number in *start..=*end {
            if check_n_exact_blocks(number) {
                count += number;
            }
        }
    }
    count
}
