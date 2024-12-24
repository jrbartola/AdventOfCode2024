pub struct Equation {
    pub test_value: u64,
    operands: Vec<u64>,
}

impl Equation {
    pub fn from_string(input: &str) -> Self {
        let mut parts = input.split(": ");
        let test_value = parts.next().map(|s| s.parse::<u64>().unwrap()).unwrap();

        let operands = parts
            .next()
            .map(|s| {
                s.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u64>>()
            })
            .unwrap();

        Self::new(test_value, operands)
    }
    fn new(test_value: u64, operands: Vec<u64>) -> Self {
        Self {
            test_value,
            operands,
        }
    }

    pub fn is_solvable_no_concat(&self) -> bool {
        fn solve_helper(test_value: &u64, operands: Vec<u64>) -> bool {
            let mut add_operands = operands.clone();
            let mut mult_operands = operands.clone();

            if operands.len() == 1 {
                return operands[0] == *test_value;
            }

            let first = operands[0];
            let second = operands[1];

            add_operands.splice(0..2, [first + second]);
            mult_operands.splice(0..2, [first * second]);

            solve_helper(test_value, add_operands) || solve_helper(test_value, mult_operands)
        }

        solve_helper(&self.test_value, self.operands.clone())
    }

    pub fn is_solvable_with_concat(&self) -> bool {
        fn solve_helper(test_value: &u64, operands: Vec<u64>) -> bool {
            let mut add_operands = operands.clone();
            let mut mult_operands = operands.clone();
            let mut concat_operands = operands.clone();

            if operands.len() == 1 {
                return operands[0] == *test_value;
            }

            let first = operands[0];
            let second = operands[1];

            add_operands.splice(0..2, [first + second]);
            mult_operands.splice(0..2, [first * second]);
            concat_operands.splice(
                0..2,
                [(first.to_string() + &second.to_string())
                    .parse::<u64>()
                    .unwrap()],
            );

            solve_helper(test_value, add_operands)
                || solve_helper(test_value, mult_operands)
                || solve_helper(test_value, concat_operands)
        }

        solve_helper(&self.test_value, self.operands.clone())
    }
}
