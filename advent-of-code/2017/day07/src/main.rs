use mkz_aoc::file;
use regex::Regex;
use std::collections::HashMap;

// I really want to rewrite this algorithm.
fn weight(leaves: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, usize>, current: &str) -> usize {
    let current_nodes = leaves.get(current);

    if current_nodes == None {
        return *weights.get(current).unwrap();
    }

    let mut current_weight = *weights.get(current).unwrap();

    for node in current_nodes.unwrap() {
        current_weight += weight(leaves, weights, node)
    }

    current_weight
}

fn is_balanced(leaves: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, usize>, current: &str) -> bool {
    let current_nodes = leaves.get(current);

    if current_nodes == None {
        return true;
    }

    let mut current_weight = 0;

    for node in current_nodes.unwrap() {
        let sub_weight = weight(leaves, weights, node);
        if current_weight == 0 {
            current_weight = sub_weight;
        } else {
            if sub_weight != current_weight {
                return false;
            }
        }
    }

    true
}

fn find_unbalanced(leaves: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, usize>, current: &str) -> Option<usize> {
    let current_nodes = leaves.get(current);

    if current_nodes == None {
        return None;
    }

    for node in current_nodes.unwrap() {
        if false == is_balanced(leaves, weights, node) {
            return find_unbalanced(leaves, weights, node);
        }
    }

    // This node is unbalanced because all children were OK. Let's find out the culprint here.
    let mut node_weights = vec![];
    let mut local_weights = vec![];
    for node in current_nodes.unwrap() {
        node_weights.push(weight(leaves, weights, node));
        local_weights.push(*weights.get(node).unwrap());
    }

    // find the "ufo" value and a valid value:
    let ufo_index = node_weights
        .iter()
        .map(|x| node_weights.iter().filter(|y| *y == x).count())
        .enumerate()
        .filter(|(_i, x)| *x == 1)
        .map(|(i, _x)| i)
        .nth(0)
        .unwrap();

    let other_index = node_weights
        .iter()
        .enumerate()
        .filter(|(i, _x)| *i != ufo_index)
        .map(|(i, _x)| i)
        .nth(0)
        .unwrap();

    // find the diff
    let diff = if node_weights[ufo_index] > node_weights[other_index] {
        local_weights[ufo_index] - (node_weights[ufo_index] - node_weights[other_index])
    } else {
        local_weights[ufo_index] + (node_weights[other_index] - node_weights[ufo_index])
    };

    Some(diff)
}

fn main() {
    let lines = file::read_to_lines("input.txt").unwrap();
    let mut some_value = "";

    let mut parents : HashMap<&str, &str> = HashMap::new();
    let mut leaves : HashMap<&str, Vec<&str>> = HashMap::new();
    let mut weights : HashMap<&str, usize> = HashMap::new();

    let re_leaf = Regex::new(r"^(.*) \((\d+)\)$").unwrap();
    let re_branch = Regex::new(r"^(.*) \((\d+)\) -> (.*)$").unwrap();

    for line in &lines {
        if !re_branch.is_match(&line) {
            let captures = re_leaf.captures(&line).unwrap();
            let parent = captures.get(1).unwrap().as_str();
            let weight = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            weights.insert(parent, weight);

            continue;
        }

        let captures = re_branch.captures(&line).unwrap();
        let parent = captures.get(1).unwrap().as_str();
        let weight = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        weights.insert(parent, weight);

        some_value = parent;

        let mut list = vec![];

        for leaf in captures.get(3).unwrap().as_str().split(", ") {
            parents.insert(leaf, parent);
            list.push(leaf);
        }

        leaves.insert(parent, list);
    }

    loop {
        if let None = parents.get(some_value) {
            break;
        }

        some_value = parents.get(some_value).unwrap();
    }

    println!("Part #1: {}", some_value);



    // println!("Total weight: {}", weight(&leaves, &weights, some_value));
    // println!("{:?}", is_balanced(&leaves, &weights, some_value));
    // println!("{:?}", find_unbalanced(&leaves, &weights, some_value));

    println!("Part #2: {}", find_unbalanced(&leaves, &weights, some_value).unwrap());
}
