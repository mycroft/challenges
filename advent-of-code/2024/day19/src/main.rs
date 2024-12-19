use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    parts: HashSet<String>,
    towels: Vec<String>,
}

fn read_input(fp: &str) -> Input {
    let contents = fs::read_to_string(fp).expect("Error reading the file!!!");

    let mut parts: HashSet<String> = HashSet::new();
    let mut towels: Vec<String> = Vec::new();

    for (line_id, line) in contents.lines().enumerate() {
        match line_id {
            0 => {
                parts = line.split(", ").map(|x| x.to_string()).collect();
            }
            1 => {}
            _ => {
                towels.push(line.to_string());
            }
        }
    }

    Input { parts, towels }
}

fn find_prefixes(parts: &HashSet<String>, towel: &str) -> Option<Vec<String>> {
    let mut prefixes: Vec<String> = Vec::new();

    for part in parts {
        if towel.starts_with(part) {
            prefixes.push(part.to_string());
        }
    }

    if !prefixes.is_empty() {
        Some(prefixes)
    } else {
        None
    }
}

fn solve_towel(cache: &mut HashMap<String, usize>, parts: &HashSet<String>, towel: String) -> usize {
    let current_towel = towel.clone();

    if towel.is_empty() {
        return 1;
    }

    let prefixes = find_prefixes(parts, &current_towel);
    if prefixes.is_none() {
        return 0;
    }

    let mut res = 0;

    for prefix in prefixes.unwrap() {
        let mut current_towel = towel.clone();
        current_towel = current_towel[prefix.len()..].to_string();

        let result;
        if cache.contains_key(&current_towel) {
            result = cache[&current_towel];
        } else {
            result = solve_towel(cache, parts, current_towel.clone());
            cache.insert(current_towel.clone(), result);
        }

        res += result;
    }

    *cache.entry(towel.clone()).or_insert(0) += res;

    cache[&towel]
}

fn solve(input: &Input) -> (usize, usize) {
    let mut cache: HashMap<String, usize> = HashMap::new();

    let mut result_step1 = 0;
    let mut result_step2 = 0;
    for towel in input.towels.iter() {
        let res = solve_towel(&mut cache, &input.parts, towel.to_string());
        if res > 0 {
            result_step1 += 1;
            result_step2 += res;
        }
    }

    (result_step1, result_step2)
}

fn main() {
    let input = read_input("input.txt");

    let result = solve(&input);
    println!("#1 {}", result.0);
    println!("#2 {}", result.1);
}

#[test]
fn sample() {
    let input = read_input("input_test.txt");
    let result = solve(&input);
    assert_eq!(result.0, 6);
    assert_eq!(result.1, 16);
}