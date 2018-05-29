use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_string = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input_string)
        .unwrap();
    input_string.trim();

    let mut input_digits = Vec::new();
    for character in input_string.chars() {
        if let Some(digit) = character.to_digit(10) {
            input_digits.push(digit);
        }
    }

    let mut sum = 0;
    let mut last_digit = input_digits[input_digits.len() - 1];
    for digit in input_digits {
        if digit == last_digit {
            sum += last_digit;
        }
        last_digit = digit;
    }

    println!("{}", sum);
}
