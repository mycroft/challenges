use std::fs::read_to_string;
use rayon::prelude::*;

// Convert number to digits once and reuse
fn get_digits(mut num: i128) -> Vec<u8> {
    if num == 0 {
        return vec![0];
    }
    
    let mut digits = Vec::new();
    while num > 0 {
        digits.push((num % 10) as u8);
        num /= 10;
    }
    digits.reverse();
    digits
}

// A number is valid if it is not made only of some sequence of digits repeated twice
fn is_valid(num: i128) -> bool {
    let digits = get_digits(num);
    let len = digits.len();
    
    if len % 2 != 0 {
        return true;
    }
    
    let mid = len / 2;
    // Compare digit slices directly instead of string conversion
    &digits[0..mid] != &digits[mid..len]
}

// A number is valid if it is not made only of some sequence of digits repeated twice or more times
fn is_valid_extended(num: i128) -> bool {
    let digits = get_digits(num);
    let len = digits.len();
    
    for sub_len in 1..=(len / 2) {
        if len % sub_len != 0 {
            continue;
        }
        
        // Check if the pattern repeats by comparing chunks directly
        let pattern = &digits[0..sub_len];
        let mut is_repeated = true;
        
        for chunk_start in (sub_len..len).step_by(sub_len) {
            if &digits[chunk_start..chunk_start + sub_len] != pattern {
                is_repeated = false;
                break;
            }
        }
        
        if is_repeated {
            return false;
        }
    }
    true
}

fn solve_step1(start: i128, end: i128) -> i128 {
    let range_size = end - start + 1;
    
    // Use parallel processing for large ranges
    if range_size > 100_000 {
        (start..=end)
            .into_par_iter()
            .filter(|&num| !is_valid(num))
            .sum()
    } else {
        (start..=end)
            .filter(|&num| !is_valid(num))
            .sum()
    }
}

fn solve_step2(start: i128, end: i128) -> i128 {
    let range_size = end - start + 1;
    
    // Use parallel processing for large ranges
    if range_size > 100_000 {
        (start..=end)
            .into_par_iter()
            .filter(|&num| !is_valid_extended(num))
            .sum()
    } else {
        (start..=end)
            .filter(|&num| !is_valid_extended(num))
            .sum()
    }
}

fn main() {
    let contents = read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let (step1, step2): (i128, i128) = contents
        .trim()
        .split(',')
        .map(|range| {
            let mut bounds = range.split('-');
            let start: i128 = bounds.next().unwrap().parse().unwrap();
            let end: i128 = bounds.next().unwrap().parse().unwrap();
            (solve_step1(start, end), solve_step2(start, end))
        })
        .fold((0, 0), |(acc1, acc2), (s1, s2)| (acc1 + s1, acc2 + s2));

    println!("#1: {}", step1);
    println!("#2: {}", step2);
}