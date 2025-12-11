use crate::Machine;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse_input(filename: &str) -> io::Result<Vec<Machine>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut machines = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed.split("}").collect();
        if parts.len() != 2 {
            continue;
        }

        let joltage_requirements_str = parts[1].trim();
        let joltage_requirements: Vec<i64> = joltage_requirements_str
            .trim_start_matches('{')
            .split(',')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        let lights_and_buttons_raw = parts[0];
        let light_diagram_raw_end = lights_and_buttons_raw.find(']').unwrap_or(0);
        let light_diagram_raw = &lights_and_buttons_raw[1..light_diagram_raw_end];
        let light_diagram: Vec<i64> = light_diagram_raw
            .chars()
            .map(|l| if l == '#' { 1 } else { 0 })
            .collect();

        let number_of_lights = light_diagram.len();

        let mut button_schematics_raw = lights_and_buttons_raw[light_diagram_raw_end + 1..].trim();
        let mut button_schematics: Vec<Vec<i64>> = Vec::new();

        while let Some(button_schematics_start) = button_schematics_raw.find('(') {
            if let Some(button_schematics_end) =
                button_schematics_raw[button_schematics_start..].find(')')
            {
                let button_indices_str = &button_schematics_raw
                    [button_schematics_start + 1..button_schematics_start + button_schematics_end];

                let button_indices: Vec<usize> = button_indices_str
                    .split(',')
                    .filter_map(|b| b.trim().parse::<usize>().ok())
                    .collect();

                let mut button_vector = vec![0; number_of_lights];

                for &idx in &button_indices {
                    if idx < number_of_lights {
                        button_vector[idx] = 1;
                    }
                }

                button_schematics.push(button_vector);

                button_schematics_raw =
                    &button_schematics_raw[button_schematics_start + button_schematics_end + 1..];
            } else {
                break;
            }
        }

        machines.push(Machine::new(
            light_diagram,
            joltage_requirements,
            button_schematics,
        ));
    }

    Ok(machines)
}
