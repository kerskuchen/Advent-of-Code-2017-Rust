use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut relations = HashMap::new();
    let input = input.replace(" <->", ",");
    for line in input.lines() {
        let ids: Vec<u32> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
        let main_id = ids[0];
        let related_ids: Vec<u32> = ids.into_iter().skip(1).collect();
        relations.insert(main_id, related_ids);
    }
    println!("{:?}", elems_in_group_of_id(0, &relations).len());
}

fn elems_in_group_of_id(id: u32, relations: &HashMap<u32, Vec<u32>>) -> HashSet<u32> {
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
