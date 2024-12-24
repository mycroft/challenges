use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Rule {
    left: String,
    right: String,
    operator: Operator,
    dest: String,
}

fn read_input(fp: &str) -> (HashMap<String, Option<bool>>, HashSet<Rule>) {
    let mut state = HashMap::new();
    let mut rules = HashSet::new();

    let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");

    let mut is_init = true;

    for line in contents.lines() {
        if line.is_empty() {
            is_init = false;
            continue;
        }

        if is_init {
            let parts: Vec<&str> = line.split(": ").collect();
            let key = parts[0].to_string();
            let value = parts[1] == "1";
            state.insert(key, Some(value));
            continue;
        }

        let parts: Vec<&str> = line.split(" -> ").collect();
        let dest = parts[1].to_string();
        let parts: Vec<&str> = parts[0].split(" ").collect();
        let left = parts[0].to_string();
        let operator = match parts[1] {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("Unknown operator"),
        };
        let right = parts[2].to_string();

        if !state.contains_key(&left) {
            state.insert(left.clone(), None);
        }
        if !state.contains_key(&right) {
            state.insert(right.clone(), None);
        }
        if !state.contains_key(&dest) {
            state.insert(dest.clone(), None);
        }

        rules.insert(Rule {
            left,
            right,
            operator,
            dest,
        });
    }

    (state, rules)
}

fn solve(state: &mut HashMap<String, Option<bool>>, rules: &HashSet<Rule>) {
    loop {
        let mut changed = false;
        for rule in rules.iter() {
            let left = match state.get(&rule.left).unwrap() {
                Some(value) => *value,
                None => continue,
            };
            let right = match state.get(&rule.right).unwrap() {
                Some(value) => *value,
                None => continue,
            };
            let result = match rule.operator {
                Operator::And => left && right,
                Operator::Or => left || right,
                Operator::Xor => left ^ right,
            };
            let dest = state.get_mut(&rule.dest).unwrap();
            if *dest != Some(result) {
                *dest = Some(result);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

fn solve_step1(state: &mut HashMap<String, Option<bool>>, rules: &HashSet<Rule>) -> u64 {
    solve(state, rules);

    let mut keys = state
        .keys()
        .filter(|x| x.starts_with("z"))
        .collect::<Vec<&String>>();
    keys.sort();
    keys.reverse();

    let mut result: u64 = 0;

    for k in keys {
        result <<= 1;
        result |= state.get(k).unwrap().unwrap() as u64;
    }

    result
}

fn get_rule(
    rules: &HashSet<Rule>,
    left: &Option<String>,
    right: &Option<String>,
    op: Operator,
) -> Option<String> {
    if left.is_none() || right.is_none() {
        return None;
    }

    let left = left.clone().unwrap();
    let right = right.clone().unwrap();

    for rule in rules {
        if (rule.left == left && rule.right == right || rule.left == right && rule.right == left)
            && rule.operator == op
        {
            return Some(rule.dest.clone());
        }
    }
    None
}

fn solve_step2(rules: &HashSet<Rule>) -> String {
    let mut carry: Option<String> = None;

    let count_z = rules.iter().filter(|x| x.dest.starts_with("z")).count();
    let mut swapped: Vec<(Option<String>, Option<String>)> = Vec::new();

    let mut new_carry;

    for i in 0..count_z - 1 {
        let mut sum_1 = get_rule(
            rules,
            &Some(format!("x{:0>2}", i)),
            &Some(format!("y{:0>2}", i)),
            Operator::Xor,
        );
        let mut carry_1 = get_rule(
            rules,
            &Some(format!("x{:0>2}", i)),
            &Some(format!("y{:0>2}", i)),
            Operator::And,
        );
        let mut sum_2 = None;

        if carry.is_some() {
            let mut carry_2 = get_rule(rules, &carry, &sum_1, Operator::And);
            if carry_2.is_none() {
                let tmp = carry_1;
                carry_1 = sum_1.clone();
                sum_1 = tmp;

                swapped.push((sum_1.clone(), carry_1.clone()));
                carry_2 = get_rule(rules, &carry, &sum_1, Operator::And);
            }

            sum_2 = get_rule(rules, &carry, &sum_1, Operator::Xor);
            if sum_1.is_some() && sum_1.clone().unwrap().starts_with("z") {
                let tmp = sum_1.clone();
                sum_1 = sum_2.clone();
                sum_2 = tmp;

                swapped.push((sum_1.clone(), sum_2.clone()));
            }

            if carry_1.is_some() && carry_1.clone().unwrap().starts_with("z") {
                let tmp = carry_1.clone();
                carry_1 = sum_2.clone();
                sum_2 = tmp;

                swapped.push((carry_1.clone(), sum_2.clone()));
            }

            if carry_2.is_some() && carry_2.clone().unwrap().starts_with("z") {
                let tmp = carry_2.clone();
                carry_2 = sum_2.clone();
                sum_2 = tmp;

                swapped.push((carry_2.clone(), sum_2.clone()));
            }

            new_carry = get_rule(rules, &carry_2, &carry_1, Operator::Or);
        } else {
            new_carry = None;
        }

        if new_carry.is_some()
            && new_carry.clone().unwrap().starts_with("z")
            && new_carry.clone().unwrap() != format!("z{:0>2}", count_z - 1)
        {
            let tmp = new_carry.clone();
            new_carry = sum_2.clone();
            sum_2 = tmp;

            swapped.push((new_carry.clone(), sum_2.clone()))
        }

        if carry.is_some() {
            carry = new_carry.clone();
        } else {
            carry = carry_1.clone();
        }
    }

    let mut res = swapped.iter().fold(Vec::new(), |mut x, z| {
        if z.0.is_some() {
            x.push(z.0.clone().unwrap());
        }
        if z.1.is_some() {
            x.push(z.1.clone().unwrap());
        }

        x
    });

    res.sort();
    res.iter().join(",")
}

fn main() {
    let (mut state, rules) = read_input("input.txt");

    let current_time = std::time::Instant::now();
    let res = solve_step1(&mut state, &rules);
    println!("#1 {}", res);
    println!("Time: {}ms", current_time.elapsed().as_millis());

    let res = solve_step2(&rules);
    println!("#2 {}", res);
    println!("Time: {}ms", current_time.elapsed().as_millis());
}
