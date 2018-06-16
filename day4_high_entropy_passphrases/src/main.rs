extern crate itertools;
use itertools::Itertools;

use std::fs::File;
use std::io::*;

fn main() {
    let lines: Vec<Vec<String>> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|word| word.to_string())
                .collect()
        })
        .collect();

    println!(
        "Number of valid passphrases: {}",
        // Count the lines where the number of unique words is the number of total words
        lines
            .iter()
            .filter(|line| line.iter().unique().count() == line.len())
            .count()
    );

    println!(
        "Number of valid anagram passphrases: {:?}",
        // Count the lines where the number of unique sorted words is the number of total words
        lines
            .iter()
            .filter(|line| line.iter()
                .map(|word| word.chars().sorted())
                .unique()
                .count() == line.len())
            .count()
    );
}
