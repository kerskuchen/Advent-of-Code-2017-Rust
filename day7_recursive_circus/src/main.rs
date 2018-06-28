extern crate itertools;
use itertools::Itertools;

use std::fs::File;
use std::io::*;

use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Disc {
    name: String,
    weight: i32,
    parent: RefCell<Weak<Disc>>,
    children: RefCell<Vec<Rc<Disc>>>,
}

fn main() {
    // Parse input
    let entries: Vec<Vec<String>> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|maybe_line| maybe_line.ok())
        .map(|line| {
            line.replace("(", " ")
                .replace(")", " ")
                .replace("->", " ")
                .replace(",", " ")
                .split_whitespace()
                .map(|part| part.to_string())
                .collect()
        })
        .collect();

    // Create discs
    let discs: HashMap<String, Rc<Disc>> = entries
        .iter()
        .map(|entry| {
            let name = entry[0].clone();
            let disc = Rc::new(Disc {
                name: name.clone(),
                weight: entry[1].parse().unwrap(),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            });
            (name, disc)
        })
        .collect();

    // Assign children to discs
    for entry in entries.iter().filter(|entry| entry.len() > 2) {
        let parent_name = &entry[0];
        let mut parent = &discs[parent_name];

        let children_names: Vec<_> = entry.iter().skip(2).collect();
        for child_name in children_names {
            let mut child = &discs[child_name];
            *child.parent.borrow_mut() = Rc::downgrade(&parent);
            parent.children.borrow_mut().push(Rc::clone(&child));
        }
    }

    // Determine root disc
    // NOTE: The root disc is the only disc without a parent
    let mut root_name = None;
    for disc in discs.values() {
        if disc.parent.borrow().upgrade().is_none() {
            root_name = Some(disc.name.clone());
            break;
        }
    }
    let root_disc = discs[&root_name.unwrap()].clone();
    println!("Root disc name: '{}'", root_disc.name);

    // Determine which one of the discs is unbalanced
    let last_unbalanced_disk = find_last_unbalanced_disk_recursive(root_disc);
    let children_tree_weights: Vec<i32> = last_unbalanced_disk
        .children
        .borrow()
        .iter()
        .map(|child| calc_disc_tree_weight_recursive(child))
        .collect();
    println!(
        "The last unbalanced discs name is: '{}'",
        last_unbalanced_disk.name
    );
    println!(
        "The child-trees of '{}' have the following weights:",
        last_unbalanced_disk.name
    );
    println!(
        "{:?}",
        last_unbalanced_disk
            .children
            .borrow()
            .iter()
            .map(|child| child.name.clone())
            .zip(children_tree_weights.iter())
            .collect::<Vec<_>>()
    );

    // Determine what the correct weight of each child-tree should be
    let correct_child_tree_weight = if (children_tree_weights[0] == children_tree_weights[1])
        || (children_tree_weights.len() == 2)
        || (children_tree_weights.len() > 2
            && (children_tree_weights[0] == children_tree_weights[2]))
    {
        children_tree_weights[0]
    } else {
        children_tree_weights[1]
    };

    // Determine which child needs to change its weight
    let (index_of_wrong_child_tree_weight, wrong_child_tree_weight) = children_tree_weights
        .iter()
        .enumerate()
        .find(|(_index, &tree_weight)| tree_weight != correct_child_tree_weight)
        .unwrap();

    let child_with_wrong_weight =
        &last_unbalanced_disk.children.borrow()[index_of_wrong_child_tree_weight];
    let child_name = &child_with_wrong_weight.name;
    let wrong_weight = child_with_wrong_weight.weight;
    println!(
        "The disc causing unbalance is called '{}' and has a weight of {}",
        child_name, wrong_weight
    );

    let correct_child_weight = correct_child_tree_weight - (wrong_child_tree_weight - wrong_weight);
    println!(
        "The correct weight for '{}' needs to be {}",
        child_name, correct_child_weight
    );
}

/// Descents down a disc tree until it finds the disc `a` for wich holds:
/// 1) Every child of `a` is balanced
/// 2) 'a' itself is not balanced
///
/// NOTE: A disc tree is balanced if all of its child-trees have the same weight
///
fn find_last_unbalanced_disk_recursive(unbalanced_disk: Rc<Disc>) -> Rc<Disc> {
    let maybe_unbalanced_child = unbalanced_disk
        .children
        .borrow()
        .iter()
        .cloned()
        .find(|child| !is_disc_tree_balanced(child));

    match maybe_unbalanced_child {
        Some(child) => find_last_unbalanced_disk_recursive(child.clone()),
        // If all of the disc's children are balanced it means the disc itself must be unbalanced
        None => unbalanced_disk,
    }
}

/// Returns true if all child-trees of `disc` have the same weight
fn is_disc_tree_balanced(disc: &Rc<Disc>) -> bool {
    let children_weights: Vec<i32> = disc.children
        .borrow()
        .iter()
        .map(|child| calc_disc_tree_weight_recursive(child))
        .sorted();
    (children_weights[0] == children_weights[children_weights.len() - 1])
}

/// Returns the weight of the whole disc-tree
fn calc_disc_tree_weight_recursive(disc: &Rc<Disc>) -> i32 {
    let mut weight = disc.weight;
    for child in disc.children.borrow().iter() {
        weight += calc_disc_tree_weight_recursive(child);
    }
    weight
}
