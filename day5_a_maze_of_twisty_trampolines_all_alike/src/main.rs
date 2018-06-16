use std::fs::File;
use std::io::*;

fn main() {
    let jump_offsets: Vec<i32> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .map(|x| x.parse().unwrap())
        .collect();

    let jump_increment_function_part1 = |x| x + 1;
    println!(
        "Number of jumps for part 1: {}",
        do_jumping(&jump_offsets, jump_increment_function_part1)
    );

    let jump_increment_function_part2 = |x| if x >= 3 { x - 1 } else { x + 1 };
    println!(
        "Number of jumps for part 2: {}",
        do_jumping(&jump_offsets, jump_increment_function_part2)
    );
}

fn do_jumping(jump_offsets: &[i32], jump_increment: fn(i32) -> i32) -> usize {
    let mut jump_offsets: Vec<_> = jump_offsets.to_vec();
    let mut num_jumps = 0;
    let mut cur_pos = 0i32;
    while (cur_pos >= 0) && ((cur_pos as usize) < jump_offsets.len()) {
        let old_pos = cur_pos;
        cur_pos += jump_offsets[cur_pos as usize];
        jump_offsets[old_pos as usize] = jump_increment(jump_offsets[old_pos as usize]);
        num_jumps += 1;
    }
    num_jumps
}
