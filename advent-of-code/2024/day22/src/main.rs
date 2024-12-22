use std::collections::{HashMap, VecDeque};

fn compute(secret: i64) -> i64 {
    let mut secret = ((secret * 64) ^ secret) % 16777216;
    secret = ((secret / 32) ^ secret) % 16777216;
    ((secret * 2048) ^ secret) % 16777216
}

fn read_input(fp: &str) -> Vec<i64> {
    std::fs::read_to_string(fp)
        .expect("Cannot read file")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn secrets(n: i64) -> Vec<i64> {
    let mut secrets = Vec::new();

    secrets.push(n);

    for i in 0..2000 {
        secrets.push(compute(secrets[i]));
    }

    secrets
}

fn prices(n: i64) -> Vec<i64> {
    let mut prices = Vec::new();

    let mut secret = n;

    prices.push(secret % 10);

    for _ in 0..2000 {
        secret = compute(secret);
        prices.push(secret % 10);
    }

    prices
}

fn changes(prices: &[i64]) -> HashMap<VecDeque<i64>, i64> {
    let mut current_change = VecDeque::new();
    let mut result = HashMap::new();

    for i in 1..prices.len() {
        current_change.push_back(prices[i] - prices[i - 1]);
        if current_change.len() > 4 {
            current_change.pop_front();
        }

        if current_change.len() < 4 {
            continue;
        }

        if result.contains_key(&current_change) {
            continue;
        }

        result.insert(current_change.clone(), prices[i]);
    }

    result
}

fn solve_step2(numbers: &Vec<i64>) -> i64 {
    let mut all_prices_changes_indexes: HashMap<VecDeque<i64>, i64> = HashMap::new();
    
    for number in numbers {
        let prices = prices(*number);
        let changes = changes(&prices);

        for (k, v) in changes.iter() {
            *all_prices_changes_indexes.entry(k.clone()).or_insert(0) += v;
        }
    }
    
    all_prices_changes_indexes.values().copied().max().unwrap()
}

fn main() {
    let input = read_input("input.txt");
    
    let secrets = input.iter().map(|x| secrets(*x)).collect::<Vec<_>>();
    let result_step1 = secrets.iter().map(|x| x[2000]).sum::<i64>();
    println!("#1 {}", result_step1);

    let result_step2 = solve_step2(&input);
    println!("#2 {}", result_step2);
}
