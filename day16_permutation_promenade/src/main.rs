use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_string = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input_string)
        .unwrap();
    let dance_moves: Vec<_> = input_string.trim().split(',').collect();

    let mut programs = String::from("abcdefghijklmnop").into_bytes();
    for dance_move in dance_moves {
        execute_dance_move(&mut programs, dance_move);
    }
    println!(
        "Programs after doing the dance: {:?}",
        std::str::from_utf8(&programs).unwrap()
    );
}

fn execute_dance_move(programs: &mut Vec<u8>, dance_move: &str) {
    let mut chars = dance_move.chars();
    match chars.next().unwrap() {
        's' => {
            let remainder: String = chars.collect();
            let split_point = remainder.parse().unwrap();
            spin(programs, split_point);
        }
        'x' => {
            let remainder: String = chars.collect();
            let numbers: Vec<_> = remainder.split('/').collect();
            let pos_a = numbers[0].parse().unwrap();
            let pos_b = numbers[1].parse().unwrap();
            exchange(programs, pos_a, pos_b);
        }
        'p' => {
            let a = chars.next().unwrap();
            let _ = chars.next().unwrap();
            let b = chars.next().unwrap();
            partner(programs, a, b);
        }
        _ => panic!("Not a dance move"),
    }
}

fn spin(programs: &mut Vec<u8>, split_point: usize) {
    let programs_len = programs.len();
    let back = programs.split_off(programs_len - split_point);
    // Prepend `back` before `programs`
    programs.splice(0..0, back.into_iter());
}

fn exchange(programs: &mut Vec<u8>, pos_a: usize, pos_b: usize) {
    let a = programs[pos_a];
    let b = programs[pos_b];
    programs[pos_a] = b;
    programs[pos_b] = a;
}

fn partner(programs: &mut Vec<u8>, a: char, b: char) {
    let a = a as u8;
    let b = b as u8;
    let pos_a = programs.iter().position(|&x| x == a).unwrap();
    let pos_b = programs.iter().position(|&x| x == b).unwrap();
    programs[pos_a] = b;
    programs[pos_b] = a;
}
