use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let digits_string: Vec<_> = line.trim().split_whitespace().collect();
        let mut digits: Vec<_> = digits_string
            .iter()
            .map(|&x| x.parse::<i32>().unwrap())
            .collect();

        digits.sort();
        let min = digits[0];
        let max = digits[digits.len() - 1];
        sum += max - min;
    }

    println!("{}", sum);
}
