use regex::Regex;

#[derive(Debug, Clone)]
pub struct Program {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    pub instructions: Vec<u8>,
}

impl Program {
    pub fn from(lines: &Vec<String>) -> Self {
        let register_a_regex = Regex::new(r"Register A: (\d+)").unwrap();
        let register_b_regex = Regex::new(r"Register B: (\d+)").unwrap();
        let register_c_regex = Regex::new(r"Register C: (\d+)").unwrap();

        let register_a_value = register_a_regex.captures(&lines[0]).unwrap()[1]
            .parse::<u64>()
            .unwrap();
        let register_b_value = register_b_regex.captures(&lines[1]).unwrap()[1]
            .parse::<u64>()
            .unwrap();
        let register_c_value = register_c_regex.captures(&lines[2]).unwrap()[1]
            .parse::<u64>()
            .unwrap();

        let instructions = lines[4]
            .split_whitespace()
            .into_iter()
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|op| op.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        Self {
            register_a: register_a_value,
            register_b: register_b_value,
            register_c: register_c_value,
            instructions,
        }
    }

    pub fn run(&mut self) -> Vec<u8> {
        let mut output_vec = Vec::new();
        let mut instruction_pointer = 0usize;

        while instruction_pointer < self.instructions.len() {
            let op_code = self.instructions[instruction_pointer];

            if let Some(output) = self.run_op_code(
                op_code,
                self.instructions[instruction_pointer + 1],
                &mut instruction_pointer,
            ) {
                output_vec.push(output);
            }
        }

        output_vec
    }

    pub fn with_register_a(&mut self, value: u64) -> &mut Self {
        self.register_a = value;
        self
    }

    fn run_op_code(
        &mut self,
        op_code: u8,
        argument: u8,
        instruction_pointer: &mut usize,
    ) -> Option<u8> {
        match op_code {
            0 => {
                // adv: Division
                let numerator = self.register_a;
                let denominator = 2u64.pow(self.get_combo_operand(argument) as u32);

                // Truncate the division and then write to the A register.
                self.register_a = numerator / denominator;
                *instruction_pointer += 2;
            }
            1 => {
                // bxl: Bitwise XOR
                self.register_b = self.register_b ^ argument as u64;
                *instruction_pointer += 2;
            }
            2 => {
                // bst: Modulo 8
                self.register_b = self.get_combo_operand(argument) % 8;
                *instruction_pointer += 2;
            }
            3 => {
                // jnz: Jump if not zero
                if self.register_a != 0 {
                    *instruction_pointer = argument as usize;
                } else {
                    *instruction_pointer += 2;
                }
            }
            4 => {
                // bxc: Bitwise XOR
                self.register_b = self.register_b ^ self.register_c;

                *instruction_pointer += 2;
            }
            5 => {
                // out: Output
                let output = self.get_combo_operand(argument) % 8;

                *instruction_pointer += 2;

                return Some(output as u8);
            }
            6 => {
                // bdv:  Division
                let numerator = self.register_a;
                let denominator = 2u64.pow(self.get_combo_operand(argument) as u32);

                // Truncate the division and then write to the A register.
                self.register_b = numerator / denominator;
                *instruction_pointer += 2;
            }
            7 => {
                // cdv:  Division
                let numerator = self.register_a;
                let denominator = 2u64.pow(self.get_combo_operand(argument) as u32);

                // Truncate the division and then write to the A register.
                self.register_c = numerator / denominator;
                *instruction_pointer += 2;
            }
            _ => panic!("Invalid op code: {}", op_code),
        }

        None
    }

    fn get_combo_operand(&self, argument: u8) -> u64 {
        match argument {
            0 | 1 | 2 | 3 => argument as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid argument: {}", argument),
        }
    }
}
