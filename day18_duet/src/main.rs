use std::collections::HashMap;
use std::fs::File;
use std::io::*;

fn main() {
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .collect();

    let mut cur_pos = 0;
    let mut registers = HashMap::new();
    let mut last_played_sound_frequency = None;
    loop {
        let instruction: Vec<_> = lines[cur_pos].split_whitespace().collect();
        let opcode = instruction[0];
        match opcode {
            "snd" => {
                let sound_frequency = extract_value(instruction[1], &mut registers);
                last_played_sound_frequency = Some(sound_frequency);
            }
            "set" => {
                let register_name = String::from(instruction[1]);
                let value = extract_value(instruction[2], &mut registers);
                let mut register_value = registers.entry(register_name).or_insert(0);
                *register_value = value;
            }
            "add" => {
                let register_name = String::from(instruction[1]);
                let value = extract_value(instruction[2], &mut registers);
                let mut register_value = registers.entry(register_name).or_insert(0);
                *register_value += value;
            }
            "mul" => {
                let register_name = String::from(instruction[1]);
                let value = extract_value(instruction[2], &mut registers);
                let mut register_value = registers.entry(register_name).or_insert(0);
                *register_value *= value;
            }
            "mod" => {
                let register_name = String::from(instruction[1]);
                let value = extract_value(instruction[2], &mut registers);
                let mut register_value = registers.entry(register_name).or_insert(0);
                *register_value %= value;
            }
            "rcv" => {
                let conditional = extract_value(instruction[1], &mut registers);
                if conditional != 0 {
                    if let Some(frequency) = last_played_sound_frequency {
                        println!("Last recovered sound frequency: {}", frequency);
                        break;
                    }
                }
            }
            "jgz" => {
                let conditional = extract_value(instruction[1], &mut registers);
                if conditional > 0 {
                    let jump_offset = extract_value(instruction[2], &mut registers);
                    cur_pos = (cur_pos as i64 + jump_offset) as usize;
                    continue;
                }
            }
            _ => panic!("Not an instruction: {}", opcode),
        }
        cur_pos += 1;
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
