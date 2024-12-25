use regex::Regex;

const CORRECTION: u64 = 10000000000000;

struct Equation {
    button_a: u32,
    button_b: u32,
    prize: u64,
}

pub struct Equations {
    x: Equation,
    y: Equation,
}

impl Equations {
    pub fn from(lines: &[String]) -> Self {
        let button_regex = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
        let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let a_coords = button_regex.captures(&lines[0]).unwrap();
        let (a_x, a_y) = (a_coords[2].parse().unwrap(), a_coords[3].parse().unwrap());

        let b_coords = button_regex.captures(&lines[1]).unwrap();
        let (b_x, b_y) = (b_coords[2].parse().unwrap(), b_coords[3].parse().unwrap());

        let x_prize = prize_regex.captures(&lines[2]).unwrap();
        let (prize_x, prize_y) = (x_prize[1].parse().unwrap(), x_prize[2].parse().unwrap());

        Self {
            x: Equation {
                button_a: a_x,
                button_b: b_x,
                prize: prize_x,
            },
            y: Equation {
                button_a: a_y,
                button_b: b_y,
                prize: prize_y,
            },
        }
    }

    pub fn from_with_correction(lines: &[String]) -> Self {
        let button_regex = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
        let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let a_coords = button_regex.captures(&lines[0]).unwrap();
        let (a_x, a_y) = (a_coords[2].parse().unwrap(), a_coords[3].parse().unwrap());

        let b_coords = button_regex.captures(&lines[1]).unwrap();
        let (b_x, b_y) = (b_coords[2].parse().unwrap(), b_coords[3].parse().unwrap());

        let x_prize = prize_regex.captures(&lines[2]).unwrap();
        let (prize_x, prize_y) = (
            x_prize[1].parse::<u64>().unwrap(),
            x_prize[2].parse::<u64>().unwrap(),
        );

        Self {
            x: Equation {
                button_a: a_x,
                button_b: b_x,
                prize: prize_x + CORRECTION,
            },
            y: Equation {
                button_a: a_y,
                button_b: b_y,
                prize: prize_y + CORRECTION,
            },
        }
    }

    pub fn solve(&self) -> Option<(u64, u64)> {
        // The system of equations is as follows:
        // prize_x = a * button_a_x + b * button_b_x
        // prize_y = a * button_a_y + b * button_b_y

        // We can solve this system of equations using Cramer's Rule.
        // The determinant of the coefficient matrix is:
        let determinant =
            (self.x.button_a * self.y.button_b) as i64 - (self.x.button_b * self.y.button_a) as i64;

        // If the determinant is zero, the system of equations has no unique solution.
        if determinant == 0 {
            return None;
        }

        // The determinant of the x matrix is:
        let determinant_x = (self.x.prize * self.y.button_b as u64) as i64
            - (self.x.button_b as u64 * self.y.prize) as i64;

        // The determinant of the y matrix is:
        let determinant_y = (self.x.button_a as i64 * self.y.prize as i64)
            - (self.x.prize as i64 * self.y.button_a as i64);

        let a = determinant_x as f64 / determinant as f64;
        let b = determinant_y as f64 / determinant as f64;

        // If a or b is not an integer, return None
        if a.fract() != 0.0 || b.fract() != 0.0 {
            return None;
        }

        // The solution is:
        Some((a as u64, b as u64))
    }
}
