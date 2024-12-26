use crate::keypad;
use crate::keypad::{DirectionalKeypadButton, Keypad, KeypadButton, NumericKeypadButton};
use common::grid::GridDirection;

pub struct Robot {
    keypad: Box<Keypad>,
    position: KeypadButton,
}

impl Robot {
    pub fn new(keypad: Keypad) -> Self {
        let starting_position = match keypad {
            Keypad::DirectionalKeypad(_) => {
                KeypadButton::DirectionalKeypadButton(DirectionalKeypadButton::A)
            }
            Keypad::NumericKeypad(_) => KeypadButton::NumericKeypadButton(NumericKeypadButton::A),
        };

        Self {
            keypad: Box::new(keypad),
            position: starting_position,
        }
    }

    pub fn press(&mut self) {
        match self.keypad.as_mut() {
            Keypad::DirectionalKeypad(directional_keypad) => {
                if let KeypadButton::DirectionalKeypadButton(button) = self.position {
                    directional_keypad.press(button);
                }
            }
            Keypad::NumericKeypad(numeric_keypad) => {
                if let KeypadButton::NumericKeypadButton(button) = self.position {
                    numeric_keypad.press(button);
                }
            }
        }
    }

    pub fn move_arm(&mut self, direction: GridDirection) -> () {
        match *self.keypad {
            Keypad::DirectionalKeypad(_) => {
                if let KeypadButton::DirectionalKeypadButton(button) = self.position {
                    let new_button =
                        keypad::DirectionalKeypad::get_button_in_direction(button, direction);

                    self.position = KeypadButton::DirectionalKeypadButton(new_button);
                } else {
                    panic!("Invalid button type for DirectionalKeypad");
                }
            }
            Keypad::NumericKeypad(_) => {
                if let KeypadButton::NumericKeypadButton(button) = self.position {
                    let new_button =
                        keypad::NumericKeypad::get_button_in_direction(button, direction);

                    self.position = KeypadButton::NumericKeypadButton(new_button);
                } else {
                    panic!("Invalid button type for NumericKeypad");
                }
            }
        }
    }

    pub fn print(&self) -> () {
        match self.keypad.as_ref() {
            Keypad::DirectionalKeypad(keypad) => {
                if let KeypadButton::DirectionalKeypadButton(button) = self.position {
                    println!("Robot is at {:?}", button);
                    keypad.print();
                }
            }
            Keypad::NumericKeypad(_) => {
                if let KeypadButton::NumericKeypadButton(button) = self.position {
                    println!("Final robot is at {:?}", button);
                }
            }
        }
    }
}
