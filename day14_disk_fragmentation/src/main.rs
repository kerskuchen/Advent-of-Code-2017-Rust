fn main() {
    let input_string = "stpzcrnm";

    // Create grid
    let mut grid = Vec::new();
    for i in 0..128 {
        let input_string = input_string.to_owned() + &format!("-{}", i);
        let knot_hash = knot_hash(&input_string);
        let mut row = knot_hash_to_grid_row(&knot_hash);
        grid.append(&mut row);
    }

    let num_filled_cells: u32 = grid.iter().sum();
    println!("Number of filled cells: {}", num_filled_cells);
}

fn knot_hash_to_grid_row(dense_hash: &[u8]) -> Vec<u32> {
    let mut result = Vec::new();
    for byte in dense_hash {
        for bit in (0..8).rev() {
            let val = if byte & (1 << bit) != 0 { 1 } else { 0 };
            result.push(val);
        }
    }
    result
}

fn knot_hash(input_string: &str) -> Vec<u8> {
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
    sparse_knot_hash_to_dense(&circular_list)
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
