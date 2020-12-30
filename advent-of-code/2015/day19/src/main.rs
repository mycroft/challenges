use rand::seq::SliceRandom;
use regex::Regex;
use std::cmp::min;
use std::collections::HashSet;
use std::fs;

fn main() {
    let _contents = fs::read_to_string("input.txt").unwrap();
    let lines = _contents.lines();

    let mut replacements: HashSet<(&str, &str)> = HashSet::new();
    let mut possibles = HashSet::new();

    let re = Regex::new(r"(.*) => (.*)").unwrap();

    let lines: Vec<&str> = lines.into_iter().collect();
    let code = lines[lines.len() - 1];

    for line in &lines {
        if line == &"" {
            break;
        }
        let cap = re.captures(line).unwrap();

        let src = cap.get(1).unwrap().as_str();
        let dst = cap.get(2).unwrap().as_str();

        replacements.insert((src, dst));
    }

    for (from, to) in &replacements {
        let matches: Vec<_> = code.match_indices(from).collect();
        for (idx, m) in &matches {
            let mut new_code = String::from("");
            new_code.push_str(&code[..*idx]);
            new_code.push_str(to);
            new_code.push_str(&code[idx + m.len()..]);

            possibles.insert(new_code.clone());
        }
    }

    println!("Part #1: {:?}", possibles.len());

    let mut min_steps = 0;
    let mut reps: Vec<&(&str, &str)> = replacements.iter().collect();

    for _it in 0..=20 {
        let mut molecule = String::from(code);
        let mut steps = 0;
        let mut changed = true;

        while molecule != "e" {
            if changed == false {
                // not changed in last iteration & molecule not found?
                molecule = String::from(code);
                reps.shuffle(&mut rand::thread_rng());
            }
            changed = false;

            for (from, to) in &reps {
                if molecule.contains(to) {
                    molecule = molecule.replacen(to, from, 1);
                    steps += 1;
                    changed = true
                }
            }
        }

        if min_steps == 0 {
            min_steps = steps;
        } else {
            min_steps = min(min_steps, steps);
        }
    }

    println!("Part #2: {:?}", min_steps);
}
