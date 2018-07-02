use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::*;

fn main() {
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .map(|line| line.replace(" <->", ","))
        .collect();

    // Create program realations
    let mut relations = HashMap::new();
    for line in lines {
        let ids: Vec<u32> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
        let main_id = ids[0];
        let related_ids: Vec<u32> = ids.into_iter().skip(1).collect();
        relations.insert(main_id, related_ids);
    }

    // Determine number of groups
    let mut num_groups_found = 0;
    let mut programs_not_in_a_group: HashSet<u32> = (0..2000).collect();
    for program_id in 0..2000 {
        let group = group_of_id(program_id, &relations);
        if group.is_subset(&programs_not_in_a_group) {
            num_groups_found += 1;
            programs_not_in_a_group = programs_not_in_a_group
                .difference(&group)
                .cloned()
                .collect();
        }
    }
    assert!(programs_not_in_a_group.is_empty());

    println!(
        "Number of programs in group that contain program ID 0: {}",
        group_of_id(0, &relations).len()
    );
    println!("Number of unique groups: {}", num_groups_found);
}

/// Returns a set of all elements that are in the same group as the program with a given ID
fn group_of_id(id: u32, relations: &HashMap<u32, Vec<u32>>) -> HashSet<u32> {
    let mut group = HashSet::new();
    let mut todo_stack = vec![id];
    while !todo_stack.is_empty() {
        let id = todo_stack.pop().unwrap();
        if !group.contains(&id) {
            let related_ids = relations.get(&id).unwrap();
            for &related_id in related_ids {
                todo_stack.push(related_id);
            }
        }
        group.insert(id);
    }
    group
}
