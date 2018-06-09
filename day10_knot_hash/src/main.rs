use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let input: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut circular_list: Vec<_> = (0..256).collect();
    let circular_list_len = circular_list.len();

    let mut cur_pos = 0;
    let mut skip_size = 0;
    for length in input {
        // Collect range out of circular-list and write it back into cirular list in reverse order
        let mut range = Vec::new();
        for index in cur_pos..(cur_pos + length) {
            let val = circular_list[index % circular_list_len];
            range.push(val);
        }
        for index in cur_pos..(cur_pos + length) {
            circular_list[index % circular_list_len] = range.pop().unwrap();
        }

        cur_pos = (cur_pos + length + skip_size) % circular_list_len;
        skip_size += 1;
    }

    println!("{}", circular_list[0] * circular_list[1]);
}
