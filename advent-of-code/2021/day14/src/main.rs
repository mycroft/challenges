#[macro_use] extern crate scan_fmt;

use std::{collections::HashMap};

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut polymer= None;

    let mut trans = HashMap::new();

    for line in lines {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }

        if polymer.is_none() {
            polymer = Some(line);
            continue;
        }

        let (from, to) = scan_fmt!(
            line,
            "{} -> {}",
            String, char
        ).unwrap();

        trans.insert(from.chars().collect::<Vec<char>>(), to);
    }

    let polymer = polymer.unwrap().chars().collect::<Vec<char>>();

    let res_1 = solve(&polymer, &trans, 10);
    println!("#1 {}", res_1);

    let res_2 = solve(&polymer, &trans, 40);
    println!("#2 {}", res_2);
}

fn solve(polymer: &[char], trans: &HashMap<Vec<char>, char>, steps: usize) -> usize {
    let mut couples = HashMap::new();

    for i in 0..(polymer.len() - 1) {
        *couples.entry([polymer[i], polymer[i + 1]].to_vec()).or_insert(0_usize) += 1;
    }

    for _ in 0..steps {
        let mut new_couples = HashMap::new();

        for (couple_k, couple_v) in couples {
            let target = *trans.get(&couple_k).unwrap();
            *new_couples.entry(
                [couple_k[0], target].to_vec()
            ).or_insert(0) += couple_v;
            *new_couples.entry(
                [target, couple_k[1]].to_vec()
            ).or_insert(0) += couple_v;
        }        

        couples = new_couples;
    }

    let mut res = HashMap::new();

    for couple in couples {
        *res.entry(couple.0[0]).or_insert(0) += couple.1
    }

    // don't forget the last letter
    *res.entry(polymer[polymer.len() - 1]).or_insert(0) += 1;

    let max_value = res.iter().map(|(_, v)| *v).max().unwrap();
    let min_value = res.iter().map(|(_, v)| *v).min().unwrap();

    max_value - min_value
}
