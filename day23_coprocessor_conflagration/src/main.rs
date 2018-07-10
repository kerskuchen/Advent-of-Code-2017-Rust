use std::collections::HashMap;
use std::fs::File;
use std::io::*;

fn main() {
    let instructions: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .collect();

    let mut program = Program::new();
    program.excecute(&instructions);
    println!(
        "Num of mul instructions invoked: {}",
        program.num_mul_instructions_invoked
    );
}

struct Program {
    registers: HashMap<String, i64>,
    cur_instruction: usize,
    num_mul_instructions_invoked: usize,
}

impl Program {
    fn new() -> Program {
        Program {
            registers: HashMap::new(),
            cur_instruction: 0,
            num_mul_instructions_invoked: 0,
        }
    }

    fn excecute(&mut self, instructions: &[String]) {
        loop {
            if self.cur_instruction >= instructions.len() {
                break;
            }

            let instruction: Vec<_> = instructions[self.cur_instruction]
                .split_whitespace()
                .collect();
            let opcode = instruction[0];
            match opcode {
                "set" => {
                    let register_name = String::from(instruction[1]);
                    let value = extract_value(instruction[2], &mut self.registers);
                    let mut register_value = self.registers.entry(register_name).or_insert(0);
                    *register_value = value;
                }
                "sub" => {
                    let register_name = String::from(instruction[1]);
                    let value = extract_value(instruction[2], &mut self.registers);
                    let mut register_value = self.registers.entry(register_name).or_insert(0);
                    *register_value -= value;
                }
                "mul" => {
                    let register_name = String::from(instruction[1]);
                    let value = extract_value(instruction[2], &mut self.registers);
                    let mut register_value = self.registers.entry(register_name).or_insert(0);
                    *register_value *= value;

                    self.num_mul_instructions_invoked += 1;
                }
                "jnz" => {
                    let conditional = extract_value(instruction[1], &mut self.registers);
                    let jump_offset = extract_value(instruction[2], &mut self.registers);
                    if conditional != 0 {
                        self.cur_instruction = (self.cur_instruction as i64 + jump_offset) as usize;
                        continue;
                    }
                }
                _ => panic!("Not an instruction: {}", opcode),
            }
            self.cur_instruction += 1;
        }
    }
}

/// If passed string ist a register name, this returns the value of the register.
/// Otherwise it returns the parsed value as a number
fn extract_value(value_or_register_name: &str, registers: &mut HashMap<String, i64>) -> i64 {
    if let Ok(value) = value_or_register_name.parse::<i64>() {
        return value;
    }
    *registers
        .entry(String::from(value_or_register_name))
        .or_insert(0)
}
