use std::collections::HashMap;
use std::fs::File;
use std::io::*;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
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

fn part_two() {
    // Original
    // ----------------
    //
    // set b 79
    // set c b
    // jnz a 2 'a
    // jnz 1 5 'b
    // 'a: mul b 100
    // sub b -100000
    // set c b
    // sub c -17000
    // 'b: set f 1
    // set d 2
    // 'e set e 2
    // 'd: set g d
    // mul g e
    // sub g b
    // jnz g 2 'c
    // set f 0
    // 'c: sub e -1
    // set g e
    // sub g b
    // jnz g -8 'd
    // sub d -1
    // set g d
    // sub g b
    // jnz g -13 'e
    // jnz f 2 'f
    // sub h -1
    // 'f: set g b
    // sub g c
    // jnz g 2 'g
    // jnz 1 3 'exit
    // 'g sub b -17
    // jnz 1 -23 'b

    // if's, variable substitution and jump labels
    // ----------------
    //
    // b = 79 * 100 + 100_000;
    // c = b + 17_000;
    // 'b: f = 1;
    // d = 2;
    // 'e e = 2;
    // 'd: g = d
    // g = g * e - b;
    // if g == 0{
    //     f = 0;
    // }
    // e += 1;
    // g = e - b;
    // jnz g -8 'd
    // d+= 1;
    // g = d-b;
    // jnz g -13 'e
    // if f==0{
    //     h+=1;
    // }
    // g =b -c;
    // if g == 0{
    // jnz 1 3 'exit
    // }
    // 'g b += 17;
    // jnz 1 -23 'b

    // more variable substitution and loops
    // ----------------
    //
    // b = 107_900;
    // c = 124_900;
    // 'main: loop {
    //     f = 1;
    //     d = 2;
    //     do {
    //         e = 2;
    //         do
    //         {
    //             g = d * e - b;
    //
    //             if g == 0 {
    //                 f = 0;
    //             }
    //
    //             e += 1;
    //             g = e - b;
    //
    //         } while g != 0
    //
    //         d += 1;
    //         g = d - b;
    //
    //     } while g != 0
    //
    //     if f == 0 {
    //         h += 1;
    //     }
    //
    //     g = b - c;
    //
    //     if g == 0 {
    //         break 'main;
    //     }
    //
    //     b += 17;
    // }

    // Getting rid of g
    // ----------------
    //
    // b = 107_900;
    // c = 124_900;
    // 'main: loop {
    //     f = 1;
    //     d = 2;
    //     do {
    //         e = 2;
    //         do
    //         {
    //             if d * e == b {
    //                 f = 0;
    //             }
    //
    //             e += 1;
    //         } while e != b
    //
    //         d += 1;
    //     } while d != b
    //
    //     if f == 0 {
    //         h += 1;
    //     }
    //
    //     if b == c {
    //         break 'main;
    //     }
    //
    //     b += 17;
    // }

    // even more variable substitution and for loops
    // ----------------
    //
    // b = 107_900;
    // c = 124_900;
    // while b <= b {
    //     f = 1;
    //     for d in 2..b {
    //         for e in 2..b {
    //             if d * e == b {
    //                 f = 0;
    //             }
    //         }
    //     }
    //     if f == 0 {
    //         h += 1;
    //     }
    //     b += 17;
    // }

    // Better semantics
    // ----------------
    // let mut h = 0;
    // let mut b = 107_900;
    // let c = 124_900;
    // while b <= b {
    //     // We increment h if b can be represented as a product of exactly two factors d,e
    //     // with d,e in [2,b[
    //     'inner: for d in 2..b {
    //         for e in 2..b {
    //             if d * e == b {
    //                 h += 1;
    //                 break 'inner;
    //             }
    //         }
    //     }
    //     b += 17;
    // }

    // Optimization and better semantics
    // ----------------
    //
    let mut h = 0;
    let mut b = 107_900;
    let c = 124_900;
    while b <= c {
        // We increment h if b is not prime
        'inner: for d in 2..b {
            if b % d == 0 {
                h += 1;
                break 'inner;
            }
        }
        b += 17;
    }

    println!("Value in register 'h' after program has finished: {}", h);
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
