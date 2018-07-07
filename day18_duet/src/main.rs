use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::*;

fn main() {
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(instructions: &[String]) {
    let mut program = Program::new(ProgramType::PartOne);
    program.excecute(instructions);
    if let Some(frequency) = program.mailbox_out.pop_back() {
        println!("Last recovered sound frequency: {}", frequency);
    }
}

fn part_two(instructions: &[String]) {
    let mut program0 = Program::new(ProgramType::PartTwo(0));
    let mut program1 = Program::new(ProgramType::PartTwo(1));
    loop {
        program0.excecute(instructions);
        program1.excecute(instructions);
        program0.mailbox_in.append(&mut program1.mailbox_out);
        program1.mailbox_in.append(&mut program0.mailbox_out);

        if program0.mailbox_in.is_empty() && program1.mailbox_in.is_empty() {
            break;
        }
    }
    println!("Program 0 sent {} messages", program0.num_messages_sent);
    println!("Program 1 sent {} messages", program1.num_messages_sent);
}

enum ProgramType {
    PartOne,
    PartTwo(i64),
}

struct Program {
    program_type: ProgramType,
    mailbox_in: VecDeque<i64>,
    mailbox_out: VecDeque<i64>,
    registers: HashMap<String, i64>,
    cur_instruction: usize,
    num_messages_sent: usize,
}

impl Program {
    fn new(program_type: ProgramType) -> Program {
        let mut result = Program {
            program_type,
            mailbox_in: VecDeque::new(),
            mailbox_out: VecDeque::new(),
            registers: HashMap::new(),
            cur_instruction: 0,
            num_messages_sent: 0,
        };
        if let ProgramType::PartTwo(program_id) = result.program_type {
            result.registers.insert(String::from("p"), program_id);
        }
        result
    }

    /// Part one: Executes given instructions until first "rcv" instruction is encountered
    ///           and its execution condition is satisfied
    /// Part two: Executes given instructions until a "rcv" instruction is
    ///           encountered with empty inbox
    fn excecute(&mut self, instructions: &[String]) {
        loop {
            let instruction: Vec<_> = instructions[self.cur_instruction]
                .split_whitespace()
                .collect();
            let opcode = instruction[0];
            match opcode {
                "snd" => {
                    let value = extract_value(instruction[1], &mut self.registers);
                    self.mailbox_out.push_back(value);
                    self.num_messages_sent += 1;
                }
                "set" => {
                    let register_name = String::from(instruction[1]);
                    let value = extract_value(instruction[2], &mut self.registers);
                    let mut register_value = self.registers.entry(register_name).or_insert(0);
                    *register_value = value;
                }
                "add" => {
                    let register_name = String::from(instruction[1]);
                    let value = extract_value(instruction[2], &mut self.registers);
                    let mut register_value = self.registers.entry(register_name).or_insert(0);
                    *register_value += value;
                }
                "mul" => {
                    let register_name = String::from(instruction[1]);
                    let value = extract_value(instruction[2], &mut self.registers);
                    let mut register_value = self.registers.entry(register_name).or_insert(0);
                    *register_value *= value;
                }
                "mod" => {
                    let register_name = String::from(instruction[1]);
                    let value = extract_value(instruction[2], &mut self.registers);
                    let mut register_value = self.registers.entry(register_name).or_insert(0);
                    *register_value %= value;
                }
                "rcv" => {
                    // NOTE: Only the receive instruction is different in each problem part
                    match self.program_type {
                        ProgramType::PartOne => {
                            let conditional = extract_value(instruction[1], &mut self.registers);
                            if conditional != 0 {
                                break;
                            }
                        }
                        ProgramType::PartTwo(_) => {
                            if let Some(value) = self.mailbox_in.pop_front() {
                                let register_name = String::from(instruction[1]);
                                let mut register_value =
                                    self.registers.entry(register_name).or_insert(0);
                                *register_value = value;
                            } else {
                                break;
                            }
                        }
                    }
                }
                "jgz" => {
                    let conditional = extract_value(instruction[1], &mut self.registers);
                    if conditional > 0 {
                        let jump_offset = extract_value(instruction[2], &mut self.registers);
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
