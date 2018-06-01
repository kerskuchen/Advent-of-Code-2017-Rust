use std::fs::File;
use std::io::Read;

use std::collections::HashSet;

fn main() {
    let mut input_string = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input_string)
        .unwrap();

    let mut list_of_nodes_with_children = HashSet::new();
    let mut list_of_children = HashSet::new();
    for line in input_string.lines() {
        // NOTE: We ignore the weight as it is not needed for this problem
        let names: String = line.chars()
            .map(|x| if x.is_ascii_alphabetic() { x } else { ' ' })
            .collect();
        let names: Vec<_> = names.split_whitespace().collect();

        if names.len() > 1 {
            // Collect all nodes that have children
            list_of_nodes_with_children.insert(String::from(names[0]));
            for &name in names.iter().skip(1) {
                list_of_children.insert(String::from(name));
            }
        } else {
            // Leaf nodes count as children
            list_of_children.insert(String::from(names[0]));
        }
    }

    // The root node is the only node with children which has no parent
    let root_node = list_of_nodes_with_children.difference(&list_of_children);
    println!("{:?}", root_node);
}
