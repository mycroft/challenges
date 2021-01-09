use regex::Regex;
use std::fs;

use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();

    let _init_assign = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    let _bot_assign = Regex::new(r"^bot (\d+) gives low to ([^ ]*) (\d+) and high to ([^ ]*) (\d+)$").unwrap();

    let mut _bots : HashMap<u32, Vec<u32>> = HashMap::new();
    let mut dispatch : Vec<u32> = vec![];

    let mut rules : Vec<&str> = vec![];
    let mut _bots_rules : HashMap<u32, &str> = HashMap::new();

    let mut outputs : HashMap<u32, u32> = HashMap::new();

    for line in lines {
        if _init_assign.is_match(line) {
            let cap = _init_assign.captures(line).unwrap();
            let value = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let bot_num = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();

            let this_bot_values = _bots.entry(bot_num).or_insert(vec![]);
            this_bot_values.push(value);

            continue;
        }

        if !_bot_assign.is_match(line) {
            println!("{:?}", line);
            break;
        }

        let cap = _bot_assign.captures(line).unwrap();
        let bot_num = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();

        _bots_rules.insert(bot_num, line);

        rules.push(line);
    }

    loop {
        let mut found = false;
        let mut bot_idx = 0;

        for v in &_bots {
            if v.1.len() == 2 && !dispatch.iter().any(|x| *x == *v.0) {
                found = true;
                bot_idx = *v.0;
                break;
            }
        }

        if !found {
            break;
        }

        let bot_rule = _bots_rules.get(&bot_idx).unwrap();
        let cap = _bot_assign.captures(bot_rule).unwrap();

        let bot_values = _bots.get(&bot_idx).unwrap();

        let min_value = *bot_values.iter().min().unwrap();
        let max_value = *bot_values.iter().max().unwrap();

        dispatch.push(bot_idx);

        if cap.get(2).unwrap().as_str() == "bot" {
            let bot_num = cap.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let this_bot_values = _bots.entry(bot_num).or_insert(vec![]);
            this_bot_values.push(min_value);
        } else {
            let output_num = cap.get(3).unwrap().as_str().parse::<u32>().unwrap();
            outputs.insert(output_num, min_value);
        }

        if cap.get(4).unwrap().as_str() == "bot" {
            let bot_num = cap.get(5).unwrap().as_str().parse::<u32>().unwrap();
            let this_bot_values = _bots.entry(bot_num).or_insert(vec![]);
            this_bot_values.push(max_value);
        } else {
            let output_num = cap.get(5).unwrap().as_str().parse::<u32>().unwrap();
            outputs.insert(output_num, max_value);
        }
    }

    for (_bot, bot_values) in _bots {
        if bot_values == [17, 61] || bot_values == [61, 17] {
            println!("Part #1: {}", _bot);
            break;
        }
    }

    let part2 = outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap();

    println!("Part #2: {}", part2);
}
