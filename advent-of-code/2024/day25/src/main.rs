use std::fs;
use std::collections::HashSet;

// returns (locks, keys)
fn read_input(fp: &str) -> (HashSet<[usize; 5]>, HashSet<[usize; 5]>) {
    let input = fs::read_to_string(fp).expect("Error reading input.txt");
    let lines = input.lines();

    let mut locks = HashSet::new();
    let mut keys = HashSet::new();

    let mut idx: usize = 0;
    let mut is_lock: Option<bool> = None;
    let mut current = [0; 5];

    for line in lines {
        if line.is_empty() {
            if is_lock.unwrap() {
                locks.insert(current);
            } else {
                keys.insert(current);
            }

            current = [0; 5];
            is_lock = None;

            continue;
        }

        if is_lock.is_none() {
            is_lock = Some(line.starts_with('#'));
            if is_lock.unwrap() {
                idx = 1;
            } else {
                idx = 5;
            }
            continue;
        }

        if idx > 5 {
            continue;
        }

        if is_lock.unwrap() {
            for (id, c) in line.chars().enumerate() {
                if c == '#' {
                    current[id] = idx;
                }
            }
            
            idx += 1;
            
            
        } else {
            for (id, c) in line.chars().enumerate() {
                if c == '#' && current[id] == 0 {
                    current[id] = idx;
                }
            }

            idx = idx.saturating_sub(1);
        }
    }

    if is_lock.unwrap() {
        locks.insert(current);
    } else {
        keys.insert(current);
    }


    (locks, keys)
}

fn fits(lock: &[usize], key: &[usize]) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] > 5 {
            return false;
        }
    }

    true
}

fn solve(locks: &HashSet<[usize; 5]>, keys: &HashSet<[usize; 5]>) -> usize {
    let mut res = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let is_fitting = fits(lock, key);
            res += is_fitting as usize;
        }
    }

    res
}

fn main() {
    let (locks, keys) = read_input("input.txt");
    let res = solve(&locks, &keys);
    println!("#1 {}", res);
}
