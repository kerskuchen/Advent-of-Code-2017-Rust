use std::fs;
use std::io::Read;

enum ParseMode {
    Group,
    Garbage,
    Ignore,
}

fn main() {
    let mut input = String::new();
    fs::File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut total_score = 0;
    let mut group_nesting_level = 0;
    let mut garbage_char_count = 0;
    let mut parse_mode = ParseMode::Group;

    for c in input.chars() {
        match parse_mode {
            ParseMode::Group => match c {
                '{' => {
                    group_nesting_level += 1;
                    total_score += group_nesting_level;
                }
                '}' => group_nesting_level -= 1,
                '<' => parse_mode = ParseMode::Garbage,
                _ => {}
            },
            ParseMode::Garbage => match c {
                '!' => parse_mode = ParseMode::Ignore,
                '>' => parse_mode = ParseMode::Group,
                _ => garbage_char_count += 1,
            },
            ParseMode::Ignore => {
                // NOTE: '!' symbols can only exist in garbage so we can safely switch back to
                //       garbage mode
                parse_mode = ParseMode::Garbage
            }
        }
    }

    println!("Total score: {}", total_score);
    println!("Garbage characters count: {}", garbage_char_count);
}
