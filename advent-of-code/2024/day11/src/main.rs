use std::{collections::{HashMap, VecDeque}, fs};

#[derive(Debug, Hash, Eq, PartialEq)]
struct CacheIndex {
    iterations: usize,
    number: u128,
}

fn read_input(fp: &str) -> VecDeque<u128> {
    let content = fs::read_to_string(fp).expect("Error reading the file");
    content
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn transform(number: u128) -> VecDeque<u128> {
    if number == 0 {
        return VecDeque::from(vec![1]);
    }

    if number.to_string().len() % 2 == 0 {
        // split the number of 2 parts of equal size
        let number_str = number.to_string();
        let half = number_str.len() / 2;
        let (left, right) = number_str.split_at(half);
        let left = left.parse().unwrap();
        let right = right.parse().unwrap();
        VecDeque::from(vec![left, right])
    } else {
        VecDeque::from(vec![number * 2024])
    }
}

fn solve_stone(cache: &mut HashMap<CacheIndex, usize>, number: u128, iterations: usize) -> usize {
    if let Some(value) = cache.get(&CacheIndex { iterations, number }) {
        return *value;
    }

    if iterations == 0 {
        return 1;
    }

    let mut num_stones = 0;
    let transformed: VecDeque<u128> = transform(number);

    for transform in transformed {
        let num_stone = solve_stone(cache, transform, iterations - 1);

        cache.insert(CacheIndex { iterations: iterations - 1, number: transform }, num_stone);

        num_stones += num_stone;
    }

    num_stones
}

fn solve_all_stones(cache: &mut HashMap<CacheIndex, usize>, numbers: &VecDeque<u128>, iterations: usize) -> usize {
    let mut num_stores = 0;
    for number in numbers {
        num_stores += solve_stone(cache, *number, iterations);
    }

    num_stores
}

fn main() {
    let mut cache = HashMap::new();

    let numbers = read_input("input.txt");

    println!("#1 {:?}", solve_all_stones(&mut cache, &numbers, 25));
    println!("#2 {:?}", solve_all_stones(&mut cache, &numbers, 75));
}
