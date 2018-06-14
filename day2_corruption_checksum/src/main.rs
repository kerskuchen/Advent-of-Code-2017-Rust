use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let rows_of_sorted_digits: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            let mut result = line.trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            result.sort();
            result
        })
        .collect();

    println!("{}", checksum_part1(&rows_of_sorted_digits));
    println!("{}", checksum_part2(&rows_of_sorted_digits));
}

fn checksum_part1(rows_of_sorted_digits: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for row in rows_of_sorted_digits {
        let min = row[0];
        let max = row[row.len() - 1];
        sum += max - min;
    }
    sum
}

fn checksum_part2(rows_of_sorted_digits: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    'rowsloop: for row in rows_of_sorted_digits {
        for i in 0..row.len() {
            for j in (i + 1)..row.len() {
                if row[i] % row[j] == 0 {
                    sum += row[i] / row[j];
                    continue 'rowsloop;
                } else if row[j] % row[i] == 0 {
                    sum += row[j] / row[i];
                    continue 'rowsloop;
                }
            }
        }
    }
    sum
}
