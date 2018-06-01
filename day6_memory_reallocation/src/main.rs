use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_text = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input_text)
        .unwrap();

    let memory_chunk: Vec<_> = input_text
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut memory_chunk_history = Vec::new();
    memory_chunk_history.push(memory_chunk);

    let mut num_redistribution_cycles = 0;
    loop {
        let mut memory_chunk = memory_chunk_history[memory_chunk_history.len() - 1].clone();
        redistribute_blocks(&mut memory_chunk);
        num_redistribution_cycles += 1;
        if does_chunk_already_exist_in_history(&memory_chunk, &memory_chunk_history) {
            break;
        }
        memory_chunk_history.push(memory_chunk);
    }

    println!("{}", num_redistribution_cycles);
}

fn redistribute_blocks(memory_banks: &mut [u32]) {
    let (mut index, mut num_blocks) = remove_bank_with_max_num_blocks(memory_banks);

    while num_blocks > 0 {
        index = (index + 1) % memory_banks.len();
        memory_banks[index] += 1;
        num_blocks -= 1;
    }
}

fn remove_bank_with_max_num_blocks(memory_banks: &mut [u32]) -> (usize, u32) {
    let mut max_num_blocks_index = 0;
    let mut max_num_blocks = memory_banks[max_num_blocks_index];

    for (index, &val) in memory_banks.iter().enumerate() {
        if max_num_blocks < val {
            max_num_blocks = val;
            max_num_blocks_index = index;
        }
    }
    memory_banks[max_num_blocks_index] = 0;
    (max_num_blocks_index, max_num_blocks)
}

fn does_chunk_already_exist_in_history(
    memory_banks_cmp: &[u32],
    memory_banks_history: &[Vec<u32>],
) -> bool {
    for memory_banks in memory_banks_history {
        if are_memory_banks_equal(memory_banks_cmp, memory_banks) {
            return true;
        }
    }
    false
}

fn are_memory_banks_equal(a: &[u32], b: &[u32]) -> bool {
    (a.len() == b.len()) && a.iter().zip(b).all(|(x, y)| x == y)
}
