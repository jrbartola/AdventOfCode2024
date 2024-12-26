mod keypad;
mod robot;
mod robot_recurse;
mod user;

use crate::robot_recurse::robot_recurse;
use common::filereader;

fn solve1(lines: &Vec<String>) -> usize {
    robot_recurse("029A".to_string())

    // let numeric_keypad = Keypad::new_numeric_keypad();
    // let robot3 = Robot::new(numeric_keypad);
    //
    // let directional_keypad2 = Keypad::new_directional_keypad(robot3);
    // let robot2 = Robot::new(directional_keypad2);
    //
    // let directional_keypad1 = Keypad::new_directional_keypad(robot2);
    // let robot1 = Robot::new(directional_keypad1);
    //
    // let user_directional_keypad = DirectionalKeypad::new(robot1);
    //
    // let mut user = User::new(user_directional_keypad);
    //
    // user.print();
    //
    // user.press(DirectionalKeypadButton::Down);
    // user.print();
    // user.press(DirectionalKeypadButton::Left);
    // user.print();
    // user.press(DirectionalKeypadButton::A);
    //
    // user.print();
}

fn solve2(lines: &Vec<String>) -> u64 {
    0
}

fn main() {
    match filereader::read_file("./day21/resources/input.txt") {
        Ok(lines) => {
            let start = std::time::Instant::now();
            let result = solve1(&lines);
            let duration = start.elapsed();
            println!("Part 1: {:?} (took {:?})", result, duration);

            let start = std::time::Instant::now();
            let result = solve2(&lines);
            let duration = start.elapsed();
            println!("Part 2: {:?} (took {:?})", result, duration);
        }
        Err(e) => panic!("{}", e),
    }
}
