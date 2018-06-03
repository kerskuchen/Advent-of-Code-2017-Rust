use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut registers = HashMap::new();
    for line in input.lines() {
        let instruction: Vec<_> = line.split_whitespace().collect();

        let condition_register_value = *registers.entry(instruction[4]).or_insert(0);
        let condition_operator = instruction[5];
        let condition_value = instruction[6].parse::<i32>().unwrap();
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
            let mut target_register_value = registers.entry(instruction[0]).or_insert(0);
            let op = instruction[1];
            let op_value = instruction[2].parse::<i32>().unwrap();
            match op {
                "inc" => *target_register_value += op_value,
                "dec" => *target_register_value -= op_value,
                _ => panic!("Invalid instruction: {}", line),
            };
        }
    }

    let max_register_value = registers.values().max().unwrap();
    println!("{}", max_register_value);
}
