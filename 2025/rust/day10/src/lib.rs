pub mod parser;

use std::iter::zip;

#[derive(Debug, Clone)]
pub struct Machine {
    pub target_light_config: Vec<i64>,
    pub joltage_requirements: Vec<i64>,
    pub button_effects: Vec<Vec<i64>>,
}

impl Machine {
    pub fn new(
        target_light_config: Vec<i64>,
        joltage_requirements: Vec<i64>,
        button_effects: Vec<Vec<i64>>,
    ) -> Self {
        Machine {
            target_light_config,
            joltage_requirements,
            button_effects,
        }
    }
}

fn min_button_presses_to_configure_lights(machine: &Machine) -> Option<i64> {
    let num_lights = machine.target_light_config.len();
    let num_buttons = machine.button_effects.len();

    if num_lights == 0 || num_buttons == 0 {
        return Some(0);
    }

    let mut augmented_matrix: Vec<Vec<i64>> = vec![vec![0; num_buttons + 1]; num_lights];

    for light_idx in 0..num_lights {
        for button_idx in 0..num_buttons {
            augmented_matrix[light_idx][button_idx] = machine.button_effects[button_idx][light_idx];
        }
        augmented_matrix[light_idx][num_buttons] = machine.target_light_config[light_idx];
    }

    let mut current_reduced_row = 0;
    for col_idx in 0..num_buttons {
        if current_reduced_row == num_lights {
            break;
        }

        let mut row_with_nonzero_entry = current_reduced_row;
        while row_with_nonzero_entry < num_lights
            && augmented_matrix[row_with_nonzero_entry][col_idx] == 0
        {
            row_with_nonzero_entry += 1;
        }

        if row_with_nonzero_entry < num_lights {
            augmented_matrix.swap(row_with_nonzero_entry, current_reduced_row);

            for row_idx in 0..num_lights {
                if row_idx != current_reduced_row && augmented_matrix[row_idx][col_idx] == 1 {
                    for col in col_idx..=num_buttons {
                        augmented_matrix[row_idx][col] = (augmented_matrix[row_idx][col]
                            + augmented_matrix[current_reduced_row][col])
                            % 2;
                    }
                }
            }
            current_reduced_row += 1;
        }
    }

    for row_idx in current_reduced_row..num_lights {
        if augmented_matrix[row_idx][num_buttons] == 1 {
            return None;
        }
    }

    let mut base_solution = vec![0; num_buttons];
    let mut button_has_unique_effect = vec![false; num_buttons];
    let mut processed_row_count = 0;

    for button_idx in 0..num_buttons {
        if processed_row_count < num_lights
            && augmented_matrix[processed_row_count][button_idx] == 1
        {
            button_has_unique_effect[button_idx] = true;
            base_solution[button_idx] = augmented_matrix[processed_row_count][num_buttons];
            processed_row_count += 1;
        }
    }

    let mut alternative_button_press_patterns: Vec<Vec<i64>> = Vec::new();
    for optional_button_idx in 0..num_buttons {
        if !button_has_unique_effect[optional_button_idx] {
            let mut alternative_pattern = vec![0; num_buttons];
            alternative_pattern[optional_button_idx] = 1;

            for row_idx in (0..current_reduced_row).rev() {
                let leading_button_in_row = augmented_matrix[row_idx][0..num_buttons]
                    .iter()
                    .position(|&val| val == 1)
                    .unwrap_or(num_buttons);

                if leading_button_in_row < num_buttons {
                    if leading_button_in_row != optional_button_idx {
                        alternative_pattern[leading_button_in_row] = (alternative_pattern
                            [leading_button_in_row]
                            + augmented_matrix[row_idx][optional_button_idx])
                            % 2;
                    }

                    for col_idx in (leading_button_in_row + 1)..num_buttons {
                        if !button_has_unique_effect[col_idx] && alternative_pattern[col_idx] == 1 {
                            alternative_pattern[leading_button_in_row] = (alternative_pattern
                                [leading_button_in_row]
                                + augmented_matrix[row_idx][col_idx])
                                % 2;
                        }
                    }
                }
            }

            for row_idx in 0..current_reduced_row {
                let leading_button_in_row = augmented_matrix[row_idx][0..num_buttons]
                    .iter()
                    .position(|&v| v == 1)
                    .unwrap_or(num_buttons);

                if leading_button_in_row < num_buttons {
                    alternative_pattern[leading_button_in_row] = (alternative_pattern
                        [leading_button_in_row]
                        + augmented_matrix[row_idx][optional_button_idx])
                        % 2;
                }
            }
            alternative_button_press_patterns.push(alternative_pattern);
        }
    }

    let num_alternative_patterns = alternative_button_press_patterns.len();
    let mut min_presses = base_solution.iter().sum::<i64>();

    for combination in 0..(1 << num_alternative_patterns) {
        let mut solution = base_solution.clone();

        for (pattern_idx, alternative_pattern) in
            alternative_button_press_patterns.iter().enumerate()
        {
            if (combination >> pattern_idx) & 1 == 1 {
                for (press_count, &coefficient) in zip(solution.iter_mut(), alternative_pattern) {
                    *press_count = (*press_count + coefficient) % 2;
                }
            }
        }

        let total_presses = solution.iter().sum::<i64>();
        min_presses = min_presses.min(total_presses);
    }

    Some(min_presses)
}

pub fn min_button_presses_to_configure_lights_all(machines: &[Machine]) -> i64 {
    machines
        .iter()
        .filter_map(min_button_presses_to_configure_lights)
        .sum()
}
