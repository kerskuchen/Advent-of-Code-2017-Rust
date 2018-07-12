#![feature(drain_filter)]

use std::fs::File;
use std::io::*;

type Component = (u32, u32);

fn main() {
    // Collect components from input
    let components: Vec<Component> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .map(|line| {
            let parts: Vec<u32> = line.trim()
                .split('/')
                .map(|part| part.parse().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect();

    // Part one
    let starting_constraint = 0;
    let starting_bridge = [];
    let max_bridge_strength = construct_bridges(
        &starting_bridge,
        components.clone(),
        starting_constraint,
        &max_strength_function,
    ).0;
    println!("Strongest bridge: {}", max_bridge_strength);

    // Part two
    let strength_of_longest_bridge = construct_bridges(
        &starting_bridge,
        components,
        starting_constraint,
        &max_length_function,
    ).0;
    println!("Strength of longest bridge: {}", strength_of_longest_bridge);
}

/// Constructs all possible bridges out of given components and returns the maximum tuple
/// (bridge_strength, bridge_length), where the maximum is defined by the given max_function
fn construct_bridges(
    bridge: &[Component],
    mut components: Vec<Component>,
    constraint: u32,
    max_function: &Fn((u32, usize), (u32, usize)) -> (u32, usize),
) -> (u32, usize) {
    let mut max_bridge_properties = (bridge_strength(bridge), bridge.len());

    let next_possible_components = extract_next_possible_components(&mut components, constraint);
    for &next_component in &next_possible_components {
        let mut new_bridge = bridge.to_owned();
        new_bridge.push(next_component);

        // NOTE: remaining_components = components + next_possible_components - next_component
        let remaining_components = next_possible_components
            .iter()
            .filter(|&&component| component != next_component)
            .chain(components.iter())
            .cloned()
            .collect();

        let new_constraint = next_component.1;
        let bridge_properties = construct_bridges(
            &new_bridge,
            remaining_components,
            new_constraint,
            max_function,
        );
        max_bridge_properties = max_function(bridge_properties, max_bridge_properties);
    }
    max_bridge_properties
}

/// Removes and returns components from a given list that match a given constraint
/// NOTE: We use a draining filter as a regular filter is somehow super slow on Rust 1.27 (nightly)
fn extract_next_possible_components(
    components: &mut Vec<Component>,
    constraint: u32,
) -> Vec<Component> {
    components
        .drain_filter(|component| component.0 == constraint || component.1 == constraint)
        .map(|component| {
            // Swap the components ports to always have the constrained component on
            // the left-hand side (index 0) and the free port on the right-hand side (index 1)
            if component.1 == constraint {
                (component.1, component.0)
            } else {
                component
            }
        })
        .collect()
}

fn bridge_strength(bridge: &[Component]) -> u32 {
    bridge
        .iter()
        .map(|component| component.0 + component.1)
        .sum()
}

fn max_strength_function(a: (u32, usize), b: (u32, usize)) -> (u32, usize) {
    if a.0 >= b.0 {
        a
    } else {
        b
    }
}

fn max_length_function(a: (u32, usize), b: (u32, usize)) -> (u32, usize) {
    if a.1 > b.1 {
        a
    } else if a.1 < b.1 {
        b
    } else {
        max_strength_function(a, b)
    }
}
