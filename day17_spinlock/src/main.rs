const STEP_SIZE: usize = 394;

fn main() {
    let mut buffer = vec![0];
    let mut cur_pos = 0;
    for insert_val in 1..=2017 {
        let next_pos = (cur_pos + STEP_SIZE) % buffer.len();
        let insert_pos = next_pos + 1;
        buffer.insert(insert_pos, insert_val);
        cur_pos = insert_pos;
    }
    println!(
        "The value after 2017 in the buffer is {}",
        buffer[(cur_pos + 1) % buffer.len()]
    );
}
