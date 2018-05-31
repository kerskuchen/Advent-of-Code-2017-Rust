use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut num_valids = 0;
    'lines: for line in input.lines() {
        let words: Vec<_> = line.trim().split_whitespace().collect();
        let mut map = HashSet::new();

        for word in &words {
            let cloned = word.clone();
            if map.get(cloned).is_some() {
                continue 'lines;
            }
            map.insert(cloned);
        }
        num_valids += 1;
    }
    println!("{}", num_valids);
}
