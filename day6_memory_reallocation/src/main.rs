use std::fs::File;
use std::io::*;

fn main() {
    let initial_memory_configuration: Vec<u32> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .take(1)
        .collect::<String>()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut cur_redistribution_cycle = 0;
    let mut configuration_history = Vec::new();
    configuration_history.push((initial_memory_configuration, cur_redistribution_cycle));

    loop {
        let new_configuration = {
            let cur_configuration = &configuration_history[configuration_history.len() - 1].0;
            do_redistribution_cycle(cur_configuration)
        };
        cur_redistribution_cycle += 1;

        if let Some((already_seen_configuration, cycle_of_occurrence)) =
            check_if_configuration_exists(&new_configuration, &configuration_history)
        {
            println!(
                "Found previous configuration {:?} which occured in cycle {} with of cycle-length of {}.",
                already_seen_configuration, cycle_of_occurrence,
                cur_redistribution_cycle - cycle_of_occurrence
            );
            println!(
                "Found a configuration that has been seen before at cycle {}",
                cur_redistribution_cycle
            );
            break;
        }
        configuration_history.push((new_configuration, cur_redistribution_cycle));
    }
}

fn do_redistribution_cycle(configuration: &[u32]) -> Vec<u32> {
    let (mut bank_index, mut num_blocks_to_redistribute) =
        get_bank_entry_with_max_num_blocks(configuration);

    let mut new_configuration: Vec<u32> = configuration.to_vec();
    new_configuration[bank_index] = 0;

    while num_blocks_to_redistribute > 0 {
        bank_index = (bank_index + 1) % new_configuration.len();
        new_configuration[bank_index] += 1;
        num_blocks_to_redistribute -= 1;
    }
    new_configuration
}

fn get_bank_entry_with_max_num_blocks(configuration: &[u32]) -> (usize, u32) {
    let (max_num_blocks_index, max_num_blocks) = configuration.iter().enumerate().fold(
        (0, configuration[0]),
        |(cur_bank_index, cur_max_num_blocks), (bank_index, &num_blocks)| {
            if cur_max_num_blocks < num_blocks {
                (bank_index, num_blocks)
            } else {
                (cur_bank_index, cur_max_num_blocks)
            }
        },
    );
    (max_num_blocks_index, max_num_blocks)
}

fn check_if_configuration_exists(
    configuration_to_compare: &[u32],
    configuration_history: &[(Vec<u32>, usize)],
) -> Option<(Vec<u32>, usize)> {
    let duplicates: Vec<_> = configuration_history
        .iter()
        .filter(|(configuration, _cycle_of_occurrence)| {
            are_configurations_equal(&configuration_to_compare, &configuration)
        })
        .collect();
    if duplicates.is_empty() {
        None
    } else {
        Some(duplicates[0].clone())
    }
}

fn are_configurations_equal(a: &[u32], b: &[u32]) -> bool {
    (a.len() == b.len()) && a.iter().zip(b).all(|(x, y)| x == y)
}
