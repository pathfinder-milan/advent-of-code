pub mod parser;

pub struct DigitLines {
    pub lines: Vec<Vec<u32>>,
}

fn output_joltage_from_n(digits: &[u32], n: usize) -> Vec<u32> {
    match n {
        0 => return Vec::new(),
        n if n >= digits.len() => return digits.to_vec(),
        _ => {}
    }

    let total_digits = digits.len();
    let mut sequence = Vec::with_capacity(n);
    let mut iterator = 0;

    while sequence.len() < n {
        let remaining_to_pick = n - sequence.len();

        if total_digits - iterator == remaining_to_pick {
            sequence.extend_from_slice(&digits[iterator..iterator + remaining_to_pick]);
            break;
        }

        let max_index = total_digits - remaining_to_pick;
        let max_digit = digits[iterator..=max_index].iter().max().unwrap();

        if let Some(offset) = digits[iterator..=max_index]
            .iter()
            .position(|&d| d == *max_digit)
        {
            sequence.push(*max_digit);
            iterator += offset + 1;
        }
    }
    sequence
}

pub fn output_joltage_from_2(data: &DigitLines) -> u64 {
    let n = 2;
    let mut answer = 0u64;

    for digits in &data.lines {
        if let Ok(number) = output_joltage_from_n(digits, n)
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
            .parse::<u64>()
        {
            answer += number;
        }
    }
    answer
}

pub fn output_joltage_from_12(data: &DigitLines) -> u64 {
    let n = 12;
    let mut answer = 0u64;

    for digits in &data.lines {
        if let Ok(number) = output_joltage_from_n(digits, n)
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
            .parse::<u64>()
        {
            answer += number;
        }
    }
    answer
}
