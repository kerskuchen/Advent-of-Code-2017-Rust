use std::fs::File;
use std::io::*;

fn main() {
    let input_string: String = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .nth(0)
        .unwrap()
        .trim()
        .to_string();

    part_one(&input_string);
    part_two(&input_string);
}

fn part_one(input_string: &str) {
    let length_sequence: Vec<u8> = input_string
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut start_pos = 0;
    let mut skip_size = 0;
    let mut circular_list: Vec<u8> = (0..=255).collect();
    sparse_knot_hash(
        &length_sequence,
        &mut circular_list,
        &mut start_pos,
        &mut skip_size,
    );

    println!(
        "circular_list[0] * circular_list[1] = {}",
        circular_list[0] as usize * circular_list[1] as usize
    );
}

fn part_two(input_string: &str) {
    let mut length_sequence: Vec<u8> = input_string.as_bytes().to_vec();
    let mut suffix: Vec<u8> = vec![17, 31, 73, 47, 23];
    length_sequence.append(&mut suffix);

    let mut circular_list: Vec<u8> = (0..=255).collect();
    let mut start_pos = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        sparse_knot_hash(
            &length_sequence,
            &mut circular_list,
            &mut start_pos,
            &mut skip_size,
        );
    }

    let dense_hash = sparse_knot_hash_to_dense(&circular_list);
    println!(
        "Knot hash of {:?} is:\n{:02x?}",
        input_string,
        dense_knot_hash_to_string(&dense_hash)
    );
}

fn dense_knot_hash_to_string(dense_hash: &[u8]) -> String {
    dense_hash.iter().fold(String::new(), |acc_string, &val| {
        acc_string + &format!("{:02x}", val)
    })
}

fn sparse_knot_hash_to_dense(sparse_hash: &[u8]) -> Vec<u8> {
    sparse_hash
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |xored_val, x| xored_val ^ x))
        .collect()
}

fn sparse_knot_hash(
    length_sequence: &[u8],
    circular_list: &mut [u8],
    start_pos: &mut usize,
    skip_size: &mut usize,
) {
    let circular_list_len = circular_list.len();

    let mut cur_pos = *start_pos;
    let mut cur_skip_size = *skip_size;
    for &length in length_sequence {
        let length: usize = length as usize;

        // Collect range out of circular-list and write it back into cirular list in reverse order
        let mut range = Vec::new();
        for index in cur_pos..(cur_pos + length) {
            let val = circular_list[(index as usize) % circular_list_len];
            range.push(val);
        }
        for index in cur_pos..(cur_pos + length) {
            circular_list[(index as usize) % circular_list_len] = range.pop().unwrap();
        }

        cur_pos = (cur_pos + length + cur_skip_size) % circular_list_len;
        cur_skip_size += 1;
    }

    *start_pos = cur_pos;
    *skip_size = cur_skip_size;
}
