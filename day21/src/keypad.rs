use crate::robot::Robot;
use common::grid::GridDirection;
use std::fmt::{Debug, Formatter};

pub enum KeypadButton {
    DirectionalKeypadButton(DirectionalKeypadButton),
    NumericKeypadButton(NumericKeypadButton),
}

/**
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
**/
#[derive(Clone, Copy)]
pub enum NumericKeypadButton {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Debug for NumericKeypadButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericKeypadButton::A => write!(f, "A"),
            NumericKeypadButton::Zero => write!(f, "0"),
            NumericKeypadButton::One => write!(f, "1"),
            NumericKeypadButton::Two => write!(f, "2"),
            NumericKeypadButton::Three => write!(f, "3"),
            NumericKeypadButton::Four => write!(f, "4"),
            NumericKeypadButton::Five => write!(f, "5"),
            NumericKeypadButton::Six => write!(f, "6"),
            NumericKeypadButton::Seven => write!(f, "7"),
            NumericKeypadButton::Eight => write!(f, "8"),
            NumericKeypadButton::Nine => write!(f, "9"),
        }
    }
}

impl NumericKeypadButton {
    pub fn from(c: char) -> NumericKeypadButton {
        match c {
            'A' => NumericKeypadButton::A,
            '0' => NumericKeypadButton::Zero,
            '1' => NumericKeypadButton::One,
            '2' => NumericKeypadButton::Two,
            '3' => NumericKeypadButton::Three,
            '4' => NumericKeypadButton::Four,
            '5' => NumericKeypadButton::Five,
            '6' => NumericKeypadButton::Six,
            '7' => NumericKeypadButton::Seven,
            '8' => NumericKeypadButton::Eight,
            '9' => NumericKeypadButton::Nine,
            _ => panic!("Invalid character for NumericKeypadButton"),
        }
    }
    fn get_coords(&self) -> (usize, usize) {
        match self {
            NumericKeypadButton::A => (3, 2),
            NumericKeypadButton::Zero => (3, 1),
            NumericKeypadButton::One => (2, 0),
            NumericKeypadButton::Two => (2, 1),
            NumericKeypadButton::Three => (2, 2),
            NumericKeypadButton::Four => (1, 0),
            NumericKeypadButton::Five => (1, 1),
            NumericKeypadButton::Six => (1, 2),
            NumericKeypadButton::Seven => (0, 0),
            NumericKeypadButton::Eight => (0, 1),
            NumericKeypadButton::Nine => (0, 2),
        }
    }
}

#[derive(Clone, Copy)]
pub enum DirectionalKeypadButton {
    A,
    Up,
    Left,
    Down,
    Right,
}

impl Debug for DirectionalKeypadButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionalKeypadButton::A => write!(f, "A"),
            DirectionalKeypadButton::Up => write!(f, "^"),
            DirectionalKeypadButton::Left => write!(f, "<"),
            DirectionalKeypadButton::Down => write!(f, "V"),
            DirectionalKeypadButton::Right => write!(f, ">"),
        }
    }
}

impl DirectionalKeypadButton {
    pub fn get_distance(&self, other: DirectionalKeypadButton) -> usize {
        let (x1, y1) = self.get_coords();
        let (x2, y2) = other.get_coords();
        ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize
    }

    fn get_coords(&self) -> (usize, usize) {
        match self {
            DirectionalKeypadButton::A => (0, 2),
            DirectionalKeypadButton::Up => (0, 1),
            DirectionalKeypadButton::Left => (1, 0),
            DirectionalKeypadButton::Down => (1, 1),
            DirectionalKeypadButton::Right => (1, 2),
        }
    }
}

/**
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
**/

pub enum Keypad {
    DirectionalKeypad(DirectionalKeypad),
    NumericKeypad(NumericKeypad),
}

pub struct DirectionalKeypad {
    robot: Robot,
}

pub struct NumericKeypad {}

impl DirectionalKeypad {
    pub fn new(robot: Robot) -> Self {
        Self { robot }
    }
    pub fn press(&mut self, button: DirectionalKeypadButton) {
        match button {
            DirectionalKeypadButton::A => {
                self.robot.press();
            }
            DirectionalKeypadButton::Up => {
                self.robot.move_arm(GridDirection::Up);
            }
            DirectionalKeypadButton::Left => {
                self.robot.move_arm(GridDirection::Left);
            }
            DirectionalKeypadButton::Down => {
                self.robot.move_arm(GridDirection::Down);
            }
            DirectionalKeypadButton::Right => {
                self.robot.move_arm(GridDirection::Right);
            }
        }
    }

    pub fn get_button_in_direction(
        button: DirectionalKeypadButton,
        direction: GridDirection,
    ) -> DirectionalKeypadButton {
        match button {
            DirectionalKeypadButton::A => match direction {
                GridDirection::Left => DirectionalKeypadButton::Up,
                GridDirection::Down => DirectionalKeypadButton::Right,
                _ => panic!("Invalid direction for button A"),
            },
            DirectionalKeypadButton::Up => match direction {
                GridDirection::Down => DirectionalKeypadButton::Down,
                GridDirection::Right => DirectionalKeypadButton::A,
                _ => panic!("Invalid direction for button Up"),
            },
            DirectionalKeypadButton::Left => match direction {
                GridDirection::Right => DirectionalKeypadButton::Down,
                _ => panic!("Invalid direction for button Left"),
            },
            DirectionalKeypadButton::Down => match direction {
                GridDirection::Up => DirectionalKeypadButton::Up,
                GridDirection::Left => DirectionalKeypadButton::Left,
                GridDirection::Right => DirectionalKeypadButton::Right,
                _ => panic!("Invalid direction for button Down"),
            },
            DirectionalKeypadButton::Right => match direction {
                GridDirection::Up => DirectionalKeypadButton::A,
                GridDirection::Left => DirectionalKeypadButton::Down,
                _ => panic!("Invalid direction for button Right"),
            },
        }
    }

    pub fn get_directions_for(
        source: DirectionalKeypadButton,
        dest: DirectionalKeypadButton,
    ) -> Vec<GridDirection> {
        let mut directions = Vec::new();
        let (x1, y1) = source.get_coords();
        let (x2, y2) = dest.get_coords();
        if x1 < x2 {
            for _ in 0..(x2 - x1) {
                directions.push(GridDirection::Right);
            }
        } else if x1 > x2 {
            for _ in 0..(x1 - x2) {
                directions.push(GridDirection::Left);
            }
        }
        if y1 < y2 {
            for _ in 0..(y2 - y1) {
                directions.push(GridDirection::Down);
            }
        } else if y1 > y2 {
            for _ in 0..(y1 - y2) {
                directions.push(GridDirection::Up);
            }
        }
        directions
    }

    pub fn get_distance(source: DirectionalKeypadButton, dest: DirectionalKeypadButton) -> usize {
        let (x1, y1) = source.get_coords();
        let (x2, y2) = dest.get_coords();
        ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize
    }

    pub fn print(&self) -> () {
        self.robot.print();
    }
}

impl NumericKeypad {
    pub fn press(&mut self, button: NumericKeypadButton) {
        match button {
            NumericKeypadButton::A => {
                println!("Pressed A");
            }
            NumericKeypadButton::Zero => {
                println!("Pressed 0");
            }
            NumericKeypadButton::One => {
                println!("Pressed 1");
            }
            NumericKeypadButton::Two => {
                println!("Pressed 2");
            }
            NumericKeypadButton::Three => {
                println!("Pressed 3");
            }
            NumericKeypadButton::Four => {
                println!("Pressed 4");
            }
            NumericKeypadButton::Five => {
                println!("Pressed 5");
            }
            NumericKeypadButton::Six => {
                println!("Pressed 6");
            }
            NumericKeypadButton::Seven => {
                println!("Pressed 7");
            }
            NumericKeypadButton::Eight => {
                println!("Pressed 8");
            }
            NumericKeypadButton::Nine => {
                println!("Pressed 9");
            }
        }
    }

    pub fn get_button_in_direction(
        button: NumericKeypadButton,
        direction: GridDirection,
    ) -> NumericKeypadButton {
        match button {
            NumericKeypadButton::A => match direction {
                GridDirection::Left => NumericKeypadButton::Zero,
                GridDirection::Up => NumericKeypadButton::Three,
                _ => panic!("Invalid direction for button A"),
            },
            NumericKeypadButton::Zero => match direction {
                GridDirection::Right => NumericKeypadButton::A,
                GridDirection::Up => NumericKeypadButton::Two,
                _ => panic!("Invalid direction for button Zero"),
            },
            NumericKeypadButton::One => match direction {
                GridDirection::Up => NumericKeypadButton::Four,
                GridDirection::Right => NumericKeypadButton::Two,
                _ => panic!("Invalid direction for button One"),
            },
            NumericKeypadButton::Two => match direction {
                GridDirection::Up => NumericKeypadButton::Five,
                GridDirection::Left => NumericKeypadButton::One,
                GridDirection::Right => NumericKeypadButton::Three,
                GridDirection::Down => NumericKeypadButton::Zero,
            },
            NumericKeypadButton::Three => match direction {
                GridDirection::Left => NumericKeypadButton::Two,
                GridDirection::Up => NumericKeypadButton::Six,
                GridDirection::Down => NumericKeypadButton::A,
                _ => panic!("Invalid direction for button Three"),
            },
            NumericKeypadButton::Four => match direction {
                GridDirection::Up => NumericKeypadButton::Seven,
                GridDirection::Right => NumericKeypadButton::Five,
                GridDirection::Down => NumericKeypadButton::One,
                _ => panic!("Invalid direction for button Four"),
            },
            NumericKeypadButton::Five => match direction {
                GridDirection::Left => NumericKeypadButton::Four,
                GridDirection::Down => NumericKeypadButton::Two,
                GridDirection::Right => NumericKeypadButton::Six,
                GridDirection::Up => NumericKeypadButton::Eight,
            },
            NumericKeypadButton::Six => match direction {
                GridDirection::Left => NumericKeypadButton::Five,
                GridDirection::Up => NumericKeypadButton::Nine,
                GridDirection::Down => NumericKeypadButton::Three,
                _ => panic!("Invalid direction for button Six"),
            },
            NumericKeypadButton::Seven => match direction {
                GridDirection::Down => NumericKeypadButton::Four,
                GridDirection::Right => NumericKeypadButton::Eight,
                _ => panic!("Invalid direction for button Seven"),
            },
            NumericKeypadButton::Eight => match direction {
                GridDirection::Left => NumericKeypadButton::Seven,
                GridDirection::Down => NumericKeypadButton::Five,
                GridDirection::Right => NumericKeypadButton::Nine,
                _ => panic!("Invalid direction for button Eight"),
            },
            NumericKeypadButton::Nine => match direction {
                GridDirection::Left => NumericKeypadButton::Eight,
                GridDirection::Down => NumericKeypadButton::Six,
                _ => panic!("Invalid direction for button Nine"),
            },
        }
    }

    pub fn get_distance(source: NumericKeypadButton, dest: NumericKeypadButton) -> usize {
        let (x1, y1) = source.get_coords();
        let (x2, y2) = dest.get_coords();
        ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize
    }

    pub fn get_directions_for(
        source: NumericKeypadButton,
        dest: NumericKeypadButton,
    ) -> Vec<GridDirection> {
        let mut directions = Vec::new();
        let (x1, y1) = source.get_coords();
        let (x2, y2) = dest.get_coords();
        if x1 < x2 {
            for _ in 0..(x2 - x1) {
                directions.push(GridDirection::Right);
            }
        } else if x1 > x2 {
            for _ in 0..(x1 - x2) {
                directions.push(GridDirection::Left);
            }
        }
        if y1 < y2 {
            for _ in 0..(y2 - y1) {
                directions.push(GridDirection::Down);
            }
        } else if y1 > y2 {
            for _ in 0..(y1 - y2) {
                directions.push(GridDirection::Up);
            }
        }
        directions
    }
}

impl Keypad {
    pub fn new_directional_keypad(robot: Robot) -> Self {
        Keypad::DirectionalKeypad(DirectionalKeypad { robot })
    }

    pub fn new_numeric_keypad() -> Self {
        Keypad::NumericKeypad(NumericKeypad {})
    }
}
