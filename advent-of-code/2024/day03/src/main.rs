use std::fs;
use regex::Regex;

fn read_input() -> String {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    contents
}

fn solve_step1(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(input);

    let mut result = 0;

    for cap in matches {
        let num1: u64 = cap[1].parse().unwrap();
        let num2: u64 = cap[2].parse().unwrap();

        result += num1 * num2;
    }

    result
}

fn solve_step2(input: &str) -> u64 {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let re2 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches: Vec<&str> = re.find_iter(input).map(|mat| mat.as_str()).collect();

    let mut result : u64 = 0;
    let mut enabled = true;

    for m in &matches {
        match *m {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => {
                if enabled {
                    let matches2 = re2.captures(m).unwrap();
                    result += matches2[1].parse::<u64>().unwrap() * matches2[2].parse::<u64>().unwrap();
                }
            }
        };
    }

    result
}

fn main() {
    let input = read_input();

    let result_step1 = solve_step1(&input);
    let result_step2 = solve_step2(&input);

    println!("#1: {}", result_step1); // 170778545
    println!("#2: {}", result_step2); // 82868252
}
