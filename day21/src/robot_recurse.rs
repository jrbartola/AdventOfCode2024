use crate::keypad;
use crate::keypad::NumericKeypadButton;

pub fn robot_recurse(key_code: String) -> usize {
    let mut curr_position = NumericKeypadButton::A;
    let mut cost = 0;

    for key in key_code.chars() {
        let button = NumericKeypadButton::from(key);
        let new_cost = numerical_cost(curr_position, button);

        curr_position = button;
        cost += new_cost;
    }

    cost
}

fn numerical_cost(source: NumericKeypadButton, dest: NumericKeypadButton) -> usize {
    // Cost is just 1 (for pressing the button) + dist
    keypad::NumericKeypad::get_distance(source, dest) + 1
}
