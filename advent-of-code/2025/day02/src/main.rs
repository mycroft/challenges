use std::fs::read_to_string;

// A number is valid if it is not made only of some sequence of digits repeated twice
fn is_valid(num: i128) -> bool {
    let s = num.to_string();
    let len = s.len();
    if !len.is_multiple_of(2) {
        return true;
    }
    let mid = len / 2;
    let first_half = &s[0..mid];
    let second_half = &s[mid..len];
    first_half != second_half
}

// A number is valid if it is not made only of some sequence of digits repeated twice or more times
fn is_valid_extended(num: i128) -> bool {
    let s = num.to_string();
    let len = s.len();
    for sub_len in 1..=(len / 2) {
        if !len.is_multiple_of(sub_len) {
            continue;
        }
        let sub_str = &s[0..sub_len];
        let mut repeated = String::new();
        for _ in 0..(len / sub_len) {
            repeated.push_str(sub_str);
        }
        if repeated == s {
            return false;
        }
    }
    true
}

fn solve_step1(start: i128, end: i128) -> i128 {
    let mut result = 0;
    for num in start..=end {
        if !is_valid(num) {
            result += num;
        }
    }

    result
}

fn solve_step2(start: i128, end: i128) -> i128 {
    let mut result = 0;
    for num in start..=end {
        if !is_valid_extended(num) {
            result += num;
        }
    }

    result
}

fn main() {
    let contents = read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut step1 = 0;
    let mut step2 = 0;

    let ranges = contents.split(',').collect::<Vec<&str>>();
    for range in ranges {
        let bounds = range.split('-').collect::<Vec<&str>>();
        let start: i128 = bounds[0].parse().unwrap();
        let end: i128 = bounds[1].parse().unwrap();
        step1 += solve_step1(start, end);
        step2 += solve_step2(start, end);
    }

    println!("#1: {}", step1);
    println!("#2: {}", step2);
}
