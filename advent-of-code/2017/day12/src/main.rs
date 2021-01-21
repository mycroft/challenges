use mkz_aoc::file;
use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let lines = file::read_to_lines("input.txt").unwrap();
    let re = Regex::new(r"(\d+) <-> (.*)").unwrap();
    let mut rules : HashMap<usize, Vec<usize>> = HashMap::new();

    let mut max_id = 0;
    let mut current_id = 0;
    let mut groups = 0;

    for line in lines {
        let caps = re.captures(&line).unwrap();

        let from_id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let to_ids = caps.get(2).unwrap().as_str().split(", ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        rules.insert(from_id, to_ids);
        max_id = from_id;
    }

    let mut set0 = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_front(0);

    loop {
        if current_id > max_id {
            break;
        }

        if set0.contains(&current_id) {
            current_id += 1;
            continue;
        }

        let mut stack = VecDeque::new();
        stack.push_front(current_id);

        loop {
            if stack.len() == 0 {
                break;
            }

            let current_node = stack.pop_back().unwrap();
            set0.insert(current_node);

            for el in rules.get(&current_node).unwrap() {
                if !set0.contains(el) {
                    stack.push_front(*el);
                }
            }
        }

        current_id += 1;
        groups += 1;

        if groups == 1 {
            println!("Part #1: {}", set0.len());
        }
    }

    println!("Part #2: {}", groups);
}
