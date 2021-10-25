use std::fs;
use std::collections::HashMap;

fn step(vectors: &HashMap<String, char>, state: &Vec<char>) -> Vec<char> {
    let mut res : String = String::new();

    res.push(state[0]);
    res.push(state[1]);

    for n in 0..=(state.len() - 5) {
        let s = state[n..n+5].iter().collect::<String>();

        if vectors.contains_key(&s) {
            res.push(*vectors.get(&s).unwrap());
        } else {
            res.push('.');
        }
    }

    res.push(state[state.len() - 2]);
    res.push(state[state.len() - 1]);

    res.push('.');

    res.chars().collect::<Vec<char>>()
}

fn get_sum(state: &Vec<char>, padding_num: i32) -> i32 {
    let mut sum : i32 = 0;

    for (idx, l) in state.iter().enumerate() {
        if *l == '#' {
            sum += idx as i32 - padding_num;
        }
    }

    sum
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let initial_state : &str = lines[0].split(" ").collect::<Vec<&str>>()[2];
    let mut vectors : HashMap<String, char> = HashMap::new();

    let mut state = initial_state.chars().collect::<Vec<char>>();

    let padding_num : i32 = 20;

    for _ in 0..padding_num {
        state.insert(0, '.');
        state.push('.');
    }

    for (n, line) in lines.iter().enumerate() {
        if n < 2 {
            continue;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();

        vectors.insert(String::from(parts[0]), parts[2].chars().nth(0).unwrap());
    }

    let mut loop_n = 0;
    let mut sum_20 : i128 = 0;
    let mut sum_100 : i128 = 0;
    let mut sum_101 : i128 = 0;

    // println!("{:3}: {} ({})", loop_n, state.iter().collect::<String>(), get_sum(&state, padding_num));

    for _ in 1..=120 {
        loop_n += 1;
        state = step(&vectors, &state);
        // println!("{:3}: {} ({})", loop_n, state.iter().collect::<String>(), get_sum(&state, padding_num));

        if loop_n == 20 {
            sum_20 = get_sum(&state, padding_num) as i128;
        }
        if loop_n == 100 {
            sum_100 = get_sum(&state, padding_num) as i128;
        }
        if loop_n == 101 {
            sum_101 = get_sum(&state, padding_num) as i128;
        }
    }

    println!("#1: {}", sum_20);
    println!("#2: {}", sum_100 + (sum_101 - sum_100) * (50000000000 - 100));
}
