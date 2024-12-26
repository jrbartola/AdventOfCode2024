use crate::keypad::{DirectionalKeypad, DirectionalKeypadButton};

pub struct User {
    keypad: DirectionalKeypad,
}

impl User {
    pub fn new(keypad: DirectionalKeypad) -> Self {
        Self { keypad }
    }

    pub fn press(&mut self, button: DirectionalKeypadButton) {
        self.keypad.press(button);
    }

    pub fn print(&self) -> () {
        self.keypad.print();
    }
}
