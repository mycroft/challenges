use mkz_aoc::{file,parse};
use std::collections::HashMap;

fn relocate(memory: &Vec<i128>) -> Vec<i128> {
    let mut memory = memory.clone();

    let mut remaining = *memory
        .iter()
        .max()
        .unwrap();

    let mut index = memory
        .iter()
        .position(|&x| x == remaining)
        .unwrap();

    let memory_size = memory.len();

    memory[index] = 0;

    loop {
        if remaining == 0 {
            break;
        }

        index += 1;
        remaining -= 1;
        memory[index % memory_size] += 1;
    }

    memory.to_vec()
}

fn solve1(memory: &Vec<i128>) -> usize {
    let mut memory = memory.clone();
    let mut count = 0;
    let mut known_states = vec![];

    known_states.push(memory.clone());

    loop {
        memory = relocate(&memory);
        count += 1;

        if known_states.iter().any(|x| *x == memory) {
            break;
        }

        known_states.push(memory.clone());
    }

    count
}

fn solve2(memory: &Vec<i128>) -> usize {
    let mut memory = memory.clone();
    let mut count = 0;
    let mut known_states = HashMap::new();
    let mut last_diff = 0;

    known_states.insert(memory.clone(), 0);

    loop {
        memory = relocate(&memory);
        count += 1;

        let last_seen = *known_states.entry(memory.clone()).or_insert(0);

        if last_seen != 0 {
            if last_diff == count - last_seen {
                break;
            }

            last_diff = count - last_seen;
        }

        known_states.insert(memory.clone(), count);
    }

    last_diff
}


fn main() {
    let line = file::read_to_string("input.txt").unwrap();
    let memory = parse::string_to_numbers(line);
    // let memory = vec![0, 2, 7, 0];

    println!("Part #1: {}", solve1(&memory));
    println!("Part #2: {}", solve2(&memory));
}

#[test]
fn example() {
    assert_eq!(vec![2, 4, 1, 2], relocate(&mut vec![0, 2, 7, 0]));
    assert_eq!(5, solve1(&vec![0, 2, 7, 0]));
    assert_eq!(4, solve2(&vec![0, 2, 7, 0]));
}