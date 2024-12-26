use crate::keypad;
use crate::keypad::{DirectionalKeypadButton, NumericKeypad, NumericKeypadButton};
use std::collections::HashMap;

pub fn robot_recurse(key_code: String) -> usize {
    let mut curr_position = NumericKeypadButton::A;
    let mut positions_by_level = HashMap::from([
        (1, DirectionalKeypadButton::A),
        (2, DirectionalKeypadButton::A),
        (3, DirectionalKeypadButton::A),
    ]);
    let mut cost = 0;

    for key in key_code.chars() {
        let button = NumericKeypadButton::from(key);
        for direction in NumericKeypad::get_directions_for(curr_position, button) {
            let new_level = 3;
            let next_button = NumericKeypad::get_button_in_direction(curr_position, direction);
            let new_cost = directional_recurse(next_button, new_level, &mut positions_by_level);

            positions_by_level.insert(new_level, next_button);

            cost += new_cost;
        }
    }

    cost
}

fn directional_recurse(
    dest: DirectionalKeypadButton,
    num_levels: usize,
    positions_by_level: &mut HashMap<usize, DirectionalKeypadButton>,
) -> usize {
    let level_position = positions_by_level.get(&num_levels).unwrap().clone();
    // Cost is just 1 (for pressing the button) + dist
    if num_levels == 1 {
        let distance = keypad::DirectionalKeypad::get_distance(level_position, dest) + 1;
        positions_by_level.insert(num_levels, dest);

        distance
    } else {
        let directions_for = keypad::DirectionalKeypad::get_directions_for(level_position, dest);

        let mut total_cost = 0;

        for direction in directions_for {
            let new_level = num_levels - 1;
            let next_button =
                keypad::DirectionalKeypad::get_button_in_direction(level_position, direction);
            let new_cost = directional_recurse(next_button, new_level, positions_by_level);

            positions_by_level.insert(new_level, next_button);

            total_cost += new_cost;
        }

        total_cost
    }
}

fn numerical_cost(source: NumericKeypadButton, dest: NumericKeypadButton) -> usize {
    // Cost is just 1 (for pressing the button) + dist
    keypad::NumericKeypad::get_distance(source, dest) + 1
}
