use std::{collections::HashMap, fs::read_to_string};

fn read_input(fp: &str) -> (Vec<i64>, Vec<i64>) {
    let contents: String = read_to_string(fp).expect("a file to open");

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in contents.lines() {
        let numbers = line.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    (left_list, right_list)
}

fn solve_step1(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let (mut left_list, mut right_list) = input.clone();

    let mut result = 0;

    left_list.sort();
    right_list.sort();

    for i in 0..left_list.len() {
        result += (left_list[i] - right_list[i]).abs();
    }

    result
}

fn solve_step2(input: &mut (Vec<i64>, Vec<i64>)) -> i64 {
    let (ref mut left_list, ref mut right_list) = input;

    let mut numbers: HashMap<i64, usize> = HashMap::new();

    for item in right_list {
        let count = numbers.entry(*item).or_insert(0);
        *count += 1;
    }

    let mut result = 0;

    for item in left_list {
        let num = *numbers.get(item).unwrap_or(&0) as i64;
        result += *item * num;
    }

    result
}

fn main() {
    let mut input = read_input("input.txt");

    let result_step1 = solve_step1(&input);
    let result_step2 = solve_step2(&mut input);

    println!("#1 {}", result_step1); // 1223326
    println!("#2 {}", result_step2); // 21070419
}
