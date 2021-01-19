use regex::Regex;
use mkz_aoc::file;
use std::collections::HashMap;

fn check(registry: &mut HashMap<String, i32>, test_reg: &str, test_sig: &str, test_val: &str) -> bool {
    let current_val = *registry.entry(test_reg.to_string()).or_insert(0);
    let test_val = test_val.parse::<i32>().unwrap();

    match test_sig {
        ">=" => current_val >= test_val,
        "!=" => current_val != test_val,
        ">" => current_val > test_val,
        "<=" => current_val <= test_val,
        "<" => current_val < test_val,
        "==" => current_val == test_val,
        _ => {
            println!("Not implemented: {}", test_sig);
            unimplemented!()
        }
    }
}

fn main() {
    let lines = file::read_to_lines("input.txt").unwrap();
    let mut registry : HashMap<String, i32> = HashMap::new();
    let mut max_ever = 0;

    let re = Regex::new(r"^(.*) (dec|inc) (-?\d+) if (.*) (.*) (-?\d+)$").unwrap();

    for line in lines {
        let caps = re.captures(&line).unwrap();

        let test_reg = caps.get(4).unwrap().as_str();
        let test_sig = caps.get(5).unwrap().as_str();
        let test_val = caps.get(6).unwrap().as_str();

        if !check(&mut registry, test_reg, test_sig, test_val) {
            continue;
        }

        let new_reg = caps.get(1).unwrap().as_str();
        let new_op = caps.get(2).unwrap().as_str();
        let new_val = caps.get(3).unwrap().as_str();
        let new_val = new_val.parse::<i32>().unwrap();

        let reg_val = registry.entry(new_reg.to_string()).or_insert(0);

        if new_op == "inc" {
            *reg_val += new_val;
        } else {
            *reg_val -= new_val;
        }

        if *reg_val > max_ever {
            max_ever = *reg_val;
        }
    }

    println!("Part #1: {:?}", registry.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1);
    println!("Part #2: {:?}", max_ever);
}
