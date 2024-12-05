use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn read_input(fp: &str) -> (HashMap<i64, HashSet<i64>>, HashMap<i64, HashSet<i64>>, Vec<Vec<i64>>) {
    let input = fs::read_to_string(fp).expect("Failed to read input file");
    let mut lines = input.lines();

    let mut afterwards_rules = HashMap::new();
    let mut backwards_rules = HashMap::new();
    let mut updates = Vec::new();

    // read rules
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split("|");

        let left = parts.next().unwrap().parse().unwrap();
        let right = parts.next().unwrap().parse().unwrap();

        afterwards_rules.entry(left).or_insert(HashSet::new()).insert(right);
        backwards_rules.entry(right).or_insert(HashSet::new()).insert(left);
    }

    for line in lines {
        updates.push(line.split(",").map(|x| x.parse().unwrap()).collect());
    }

    (afterwards_rules, backwards_rules, updates)
}

fn check_update(afterwards_rules: &HashMap<i64, HashSet<i64>>, backwards_rules: &HashMap<i64, HashSet<i64>>, update: &[i64]) -> bool {
    let mut is_valid = true;

    for index in 0..update.len() {
        // we check element at index is before all other elements after it
        for jndex in index+1..update.len() {
            if !afterwards_rules.contains_key(&update[index]) {
                continue;
            }

            if !afterwards_rules.get(&update[index]).unwrap().contains(&update[jndex]) {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            break;
        }

        // println!("back: {:?}", backwards_rules);
        for jndex in 0..index {
            // println!("jndex: {} value: {}", jndex, update[jndex]);
            if !backwards_rules.contains_key(&update[jndex]) {
                continue;
            }

            if backwards_rules.get(&update[jndex]).unwrap().contains(&update[index]) {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            break;
        }
    }

    is_valid
}

fn fix_update(afterwards_rules: &HashMap<i64, HashSet<i64>>, backwards_rules: &HashMap<i64, HashSet<i64>>, update: &[i64]) -> Vec<i64> {
    let mut fixed_update = Vec::new();

    for item in update {
        let mut placed = false;
    
        if fixed_update.is_empty() {
            fixed_update.push(*item);
            continue;
        }

        for jndex in 0..fixed_update.len() {
            fixed_update.insert(jndex, *item);
            if check_update(afterwards_rules, backwards_rules, &fixed_update) {
                placed = true;
                break;
            }

            fixed_update.remove(jndex);
        }

        if !placed {
            fixed_update.push(*item);
        }
    
        if !check_update(afterwards_rules, backwards_rules, &fixed_update) {
            break;
        }
    }
    
    fixed_update
}

fn solve_step1(afterwards_rules: &HashMap<i64, HashSet<i64>>, backwards_rules: &HashMap<i64, HashSet<i64>>, updates: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for update in updates {
        if check_update(afterwards_rules, backwards_rules, update) {
            result += update[(update.len() - 1) / 2];
        }
    }

    result
}

fn solve_step2(afterwards_rules: &HashMap<i64, HashSet<i64>>, backwards_rules: &HashMap<i64, HashSet<i64>>, updates: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for update in updates {
        if check_update(afterwards_rules, backwards_rules, update) {
            continue;
        }

        let fixed_update = fix_update(afterwards_rules, backwards_rules, update);
        result += fixed_update[(fixed_update.len() - 1) / 2];
    }

    result
}

fn main() {
    let (afterwards_rules, backwards_rules, updates) = read_input("input.txt");
    let result_step1 = solve_step1(&afterwards_rules, &backwards_rules, &updates);
    let result_step2 = solve_step2(&afterwards_rules, &backwards_rules, &updates);

    println!("#1: {}", result_step1); // 4281
    println!("#2: {}", result_step2); // 5466
}
