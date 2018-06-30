use std::collections::HashMap;
use std::fs::File;
use std::io::*;

fn main() {
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .collect();

    let mut highest_register_value_ever = 0;
    let mut registers = HashMap::new();
    for line in &lines {
        let instruction: Vec<_> = line.split_whitespace().collect();

        // Parse instruction
        // NOTE: We ignore instruction[3] == "if"
        let target_register_name = instruction[0];
        let op = instruction[1];
        let op_value = instruction[2].parse::<i32>().unwrap();
        let condition_register_name = instruction[4];
        let condition_operator = instruction[5];
        let condition_value = instruction[6].parse::<i32>().unwrap();

        // Parse condition
        let condition_register_value = *registers.entry(condition_register_name).or_insert(0);
        let condition = match condition_operator {
            "<" => (condition_register_value < condition_value),
            ">" => (condition_register_value > condition_value),
            "<=" => (condition_register_value <= condition_value),
            ">=" => (condition_register_value >= condition_value),
            "==" => (condition_register_value == condition_value),
            "!=" => (condition_register_value != condition_value),
            _ => panic!("Invalid instruction: {}", line),
        };

        if condition {
            let mut target_register_value = registers.entry(target_register_name).or_insert(0);
            match op {
                "inc" => *target_register_value += op_value,
                "dec" => *target_register_value -= op_value,
                _ => panic!("Invalid instruction: {}", line),
            };
        }

        highest_register_value_ever = i32::max(
            highest_register_value_ever,
            *registers.values().max().unwrap(),
        );
    }

    let highest_register_value_after_completion = registers.values().max().unwrap();
    println!(
        "Highest register value after completion: {}",
        highest_register_value_after_completion
    );

    println!(
        "Highest register value ever: {}",
        highest_register_value_ever
    );
}
