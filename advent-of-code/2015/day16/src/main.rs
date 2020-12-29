use std::collections::HashMap;
use std::fs;

use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let lines = content.lines();

    let re = Regex::new(r"^Sue (\d+): (.*)$").unwrap();
    let re_param = Regex::new(r"^(.*): (\d+)$").unwrap();

    let mut requirements = HashMap::new();
    requirements.insert("children", 3);
    requirements.insert("cats", 7);
    requirements.insert("samoyeds", 2);
    requirements.insert("pomeranians", 3);
    requirements.insert("akitas", 0);
    requirements.insert("vizslas", 0);
    requirements.insert("goldfish", 5);
    requirements.insert("trees", 3);
    requirements.insert("cars", 2);
    requirements.insert("perfumes", 1);

    for line in lines {
        let cap = re.captures(line).unwrap();
        let sue_number = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();

        let params = cap.get(2).unwrap().as_str().split(", ");

        let mut qualify_part1 = true;
        let mut qualify_part2 = true;

        for param in params {
            let cap_param = re_param.captures(param).unwrap();
            let param_name = cap_param.get(1).unwrap().as_str();
            let param_value = cap_param.get(2).unwrap().as_str().parse::<u32>().unwrap();

            if !requirements.contains_key(param_name) {
                continue;
            }

            let part1_valid = *requirements.get(param_name).unwrap() == param_value;

            let part2_valid = match param_name {
                "cats" | "tree" => {
                    *requirements.get(param_name).unwrap() < param_value
                },
                "pomeranians" | "goldfish" => {
                    *requirements.get(param_name).unwrap() > param_value
                },
                param_name => {
                    *requirements.get(param_name).unwrap() == param_value
                }
            };

            if !part1_valid {
                qualify_part1 = false;
            }

            if !part2_valid {
                qualify_part2 = false;
            }
        }

        if qualify_part1 {
            println!("Part #1: {:?}", sue_number);
        }
        if qualify_part2 {
            println!("Part #2: {:?}", sue_number);
        }
    }

}
