use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let digits: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    println!("Part 1: {}", captcha_solution(1, &digits));
    println!("Part 2: {}", captcha_solution(digits.len() / 2, &digits));
}

fn captcha_solution(step_distance: usize, digits: &[u32]) -> u32 {
    let mut sum = 0;
    for index in 0..digits.len() {
        if digits[index] == digits[(index + step_distance) % digits.len()] {
            sum += digits[index];
        }
    }
    sum
}
