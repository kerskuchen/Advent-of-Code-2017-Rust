const STEP_SIZE: usize = 394;

fn main() {
    let mut buffer = vec![0];
    let mut cur_pos = 0;

    // Insert the first 2017 values
    for insert_val in 1..=2017 {
        let next_pos = (cur_pos + STEP_SIZE) % buffer.len();
        let insert_pos = next_pos + 1;
        buffer.insert(insert_pos, insert_val);
        cur_pos = insert_pos;
    }
    println!(
        "After 2017 insertions the value following 2017 is {}",
        buffer[(cur_pos + 1) % buffer.len()]
    );

    // Insert the remaining values till 50,000,000 and check the value after the 0th index
    //
    // NOTE: Zero is always at the 0th index due to the way values are inserted into the buffer.
    //       We can use this property to just skip all insertions that are not placed directly
    //       behind the 0th index.
    for insert_val in 2018..=50_000_000 {
        let next_pos = (cur_pos + STEP_SIZE) % buffer.len();
        let insert_pos = next_pos + 1;
        if insert_pos == 1 {
            // Only actually insert a value when it is a number directly following zero
            buffer.insert(insert_pos, insert_val);
        } else {
            // We push back fake values to have a correct buffer length at every iteration
            buffer.push(0);
        }
        cur_pos = insert_pos;
    }
    println!(
        "After 50,000,000 insertions the value after 0 in the buffer is {}",
        buffer[1]
    );
}
