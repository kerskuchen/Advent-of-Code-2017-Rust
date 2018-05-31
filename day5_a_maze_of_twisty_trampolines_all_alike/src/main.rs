use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut instructions: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut num_jumps = 0;
    let mut pos = 0i32;
    while (pos >= 0) && ((pos as usize) < instructions.len()) {
        let old_pos = pos;
        pos += instructions[pos as usize];
        instructions[old_pos as usize] += 1;
        num_jumps += 1;
    }

    println!("{:?}", num_jumps);
}
